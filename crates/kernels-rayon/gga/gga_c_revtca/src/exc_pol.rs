//! GGA_C_REVTCA exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_revtca.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_revtca_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * t7;
            let t9 = (simd::cbrt(t5));
            let t10 = t9 * t9;
            let t11 = ((t6).select(t8, t10));
            let t12 = f64x8::splat(1.0) - t4;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(t12));
            let t15 = t14 * t14;
            let t16 = ((t13).select(t8, t15));
            let t18 = t11 / f64x8::splat(2.0) + t16 / f64x8::splat(2.0);
            let t19 = t18 * t18;
            let t20 = t19 * t18;
            let t21 = f64x8::splat(M_CBRT3);
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t25 * t25;
            let t27 = (simd::cbrt(t2));
            let t32 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t24 * t26 / t27;
            let t33 = (simd::atan(t32));
            let t35 = -f64x8::splat(0.655868) * t33 + f64x8::splat(0.897889);
            let t36 = t20 * t35;
            let t37 = t21 * t21;
            let t38 = f64x8::splat(1.0) / t23;
            let t39 = t37 * t38;
            let t40 = t36 * t39;
            let t41 = t25 * t27;
            let t42 = f64x8::splat(M_CBRT6);
            let t43 = t42 * t42;
            let t44 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t45 = (simd::cbrt(t44));
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t43 * t46;
            let t48 = f64x8::splat(M_CBRT2);
            let t50 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t51 = ((t50).sqrt());
            let t52 = t48 * t51;
            let t53 = t27 * t2;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = t47 * t52 * t54;
            let t57 = (simd::pow(t56, f64x8::splat(2.3)));
            let t59 = f64x8::splat(1.0) + f64x8::splat(0.004712150703442276) * t57;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t1 * t1;
            let t62 = t61 * t61;
            let t63 = t2 * t2;
            let t64 = t63 * t63;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t62 * t65;
            let t67 = f64x8::splat(M_CBRTPI);
            let t69 = (simd::cbrt(f64x8::splat(9.0)));
            let t71 = t67 * f64x8::splat(M_PI) * t69 * t47;
            let t73 = t3 * t37 * t38;
            let t76 = t71 * t52 * t73 / f64x8::splat(36.0);
            let t77 = ((f64x8::splat(f64::EPSILON)).sqrt().sqrt());
            let t78 = (t76).simd_le(t77);
            let t79 = t67 * t67;
            let t81 = t69 * t69;
            let t83 = t45 * t45;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t42 * t84;
            let t86 = t79 * t44 * t81 * t85;
            let t87 = t48 * t48;
            let t88 = t87 * t50;
            let t89 = f64x8::splat(1.0) / t63;
            let t91 = t23 * t23;
            let t92 = f64x8::splat(1.0) / t91;
            let t97 = t44 * t44;
            let t104 = t67 * t97 * f64x8::splat(M_PI) * t69 * t43 / t45 / t44;
            let t105 = t50 * t50;
            let t106 = t48 * t105;
            let t107 = t65 * t37;
            let t109 = f64x8::splat(1.0) / t23 / t22;
            let t110 = t107 * t109;
            let t114 = t97 * t44;
            let t115 = t105 * t50;
            let t116 = t114 * t115;
            let t117 = t64 * t63;
            let t118 = f64x8::splat(1.0) / t117;
            let t122 = (t77).simd_lt(t76);
            let t123 = ((t122).select(t76, t77));
            let t124 = (simd::sin(t123));
            let t125 = f64x8::splat(1.0) / t123;
            let t126 = t124 * t125;
            let t127 = ((t78).select(f64x8::splat(1.0) - t86 * t88 * t89 * t21 * t92 / f64x8::splat(432.0) + t104 * t106 * t110 / f64x8::splat(34560.0) - t116 * t118 / f64x8::splat(322560.0), t126));
            let t128 = t127 * t127;
            let t129 = f64x8::splat(1.0) - t128;
            let t131 = -t66 * t129 + f64x8::splat(1.0);
            let t132 = t60 * t131;
            let t134 = t40 * t41 * t132;
            let tzk0 = t134 / f64x8::splat(3.0);
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
