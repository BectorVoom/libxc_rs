//! GGA_X_Q2D exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q2d.c`
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
pub fn gga_x_q2d_exc_pol(
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t40 = t33 * v_sigma0 * t38;
            let t42 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t40;
            let t45 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t42;
            let t46 = t28 * t28;
            let t48 = f64x8::splat(1.0) / t30 / t29;
            let t49 = t46 * t48;
            let t50 = v_sigma0 * v_sigma0;
            let t51 = t34 * t34;
            let t52 = t51 * v_rho0;
            let t54 = f64x8::splat(1.0) / t35 / t52;
            let t58 = f64x8::splat(100.0) - t49 * t50 * t54 / f64x8::splat(576.0);
            let t60 = f64x8::splat(1.0) / t30;
            let t61 = t46 * t60;
            let t62 = ((v_sigma0).sqrt());
            let t64 = f64x8::splat(1.0) / t35 / v_rho0;
            let t66 = t61 * t62 * t64;
            let t67 = (simd::pow(t66, f64x8::splat(3.5)));
            let t69 = f64x8::splat(1.0) + t40 / f64x8::splat(24.0);
            let t72 = t45 * t58 + f64x8::splat(8.715382969798257e-05) * t67 * t69;
            let t73 = t27 * t72;
            let t74 = t29 * t29;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t50 * v_sigma0;
            let t78 = t51 * t51;
            let t79 = f64x8::splat(1.0) / t78;
            let t82 = f64x8::splat(100.0) + t75 * t76 * t79 / f64x8::splat(2304.0);
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t73 * t83;
            let t87 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t84));
            let t88 = (v_rho1).simd_le(dens_threshold);
            let t89 = -t16;
            let t91 = ((t14).select(t11, (t10).select(t15, t89 * t7)));
            let t92 = f64x8::splat(1.0) + t91;
            let t93 = (t92).simd_le(zeta_threshold);
            let t94 = (simd::cbrt(t92));
            let t96 = ((t93).select(t22, t94 * t92));
            let t97 = t5 * t96;
            let t98 = v_rho1 * v_rho1;
            let t99 = (simd::cbrt(v_rho1));
            let t100 = t99 * t99;
            let t102 = f64x8::splat(1.0) / t100 / t98;
            let t104 = t33 * v_sigma2 * t102;
            let t106 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t104;
            let t109 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t106;
            let t110 = v_sigma2 * v_sigma2;
            let t111 = t98 * t98;
            let t112 = t111 * v_rho1;
            let t114 = f64x8::splat(1.0) / t99 / t112;
            let t118 = f64x8::splat(100.0) - t49 * t110 * t114 / f64x8::splat(576.0);
            let t120 = ((v_sigma2).sqrt());
            let t122 = f64x8::splat(1.0) / t99 / v_rho1;
            let t124 = t61 * t120 * t122;
            let t125 = (simd::pow(t124, f64x8::splat(3.5)));
            let t127 = f64x8::splat(1.0) + t104 / f64x8::splat(24.0);
            let t130 = t109 * t118 + f64x8::splat(8.715382969798257e-05) * t125 * t127;
            let t131 = t27 * t130;
            let t132 = t110 * v_sigma2;
            let t134 = t111 * t111;
            let t135 = f64x8::splat(1.0) / t134;
            let t138 = f64x8::splat(100.0) + t75 * t132 * t135 / f64x8::splat(2304.0);
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t131 * t139;
            let t143 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t97 * t140));
            let tzk0 = t87 + t143;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
