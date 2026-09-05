//! LDA_X_REL vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_rel.c`
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
pub fn lda_x_rel_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t8 = v_rho0 * t7;
            let t10 = (f64x8::splat(2.0) * t8).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t12 = t11 * zeta_threshold;
            let t13 = f64x8::splat(M_CBRT2);
            let t14 = t13 * v_rho0;
            let t15 = (simd::cbrt(t8));
            let t19 = ((t10).select(t12, f64x8::splat(2.0) * t14 * t7 * t15));
            let t20 = (simd::cbrt(t6));
            let t24 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t19 * t20));
            let t25 = (v_rho1).simd_le(dens_threshold);
            let t26 = v_rho1 * t7;
            let t28 = (f64x8::splat(2.0) * t26).simd_le(zeta_threshold);
            let t29 = t13 * v_rho1;
            let t30 = (simd::cbrt(t26));
            let t34 = ((t28).select(t12, f64x8::splat(2.0) * t29 * t7 * t30));
            let t38 = ((t25).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t34 * t20));
            let t39 = t24 + t38;
            let t40 = (simd::cbrt(f64x8::splat(9.0)));
            let t41 = t40 * t40;
            let t42 = t41 * t2;
            let t43 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t44 = (simd::cbrt(t43));
            let t45 = t44 * t44;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t20 * t20;
            let t51 = f64x8::splat(1.0) + f64x8::splat(3.8075239991386495e-05) * t42 * t46 * t47;
            let t52 = ((t51).sqrt());
            let t53 = t52 * t41;
            let t54 = t2 * t44;
            let t59 = t2 * t2;
            let t60 = t40 * t59;
            let t61 = f64x8::splat(1.0) / t44;
            let t65 = (simd::ln(f64x8::splat(0.0035625477770544352) * t60 * t61 * t20 + ((((f64x8::splat(0.0035625477770544352) * t60 * t61 * t20) * (f64x8::splat(0.0035625477770544352) * t60 * t61 * t20)) + f64x8::splat(1.0)).sqrt())));
            let t66 = t65 * t40;
            let t67 = t59 * t45;
            let t68 = f64x8::splat(1.0) / t47;
            let t72 = f64x8::splat(10.396221848752237) * t53 * t54 / t20 - f64x8::splat(972.7328585562606) * t66 * t67 * t68;
            let t73 = t72 * t72;
            let t75 = f64x8::splat(1.0) - f64x8::splat(1.5) * t73;
            let tzk0 = t39 * t75;
            acc_zk = tzk0;
            let t76 = t13 * t7;
            let t79 = t6 * t6;
            let t80 = f64x8::splat(1.0) / t79;
            let t83 = f64x8::splat(2.0) * t14 * t80 * t15;
            let t84 = t15 * t15;
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t7 * t85;
            let t88 = -v_rho0 * t80 + t7;
            let t93 = ((t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t76 * t15 - t83 + f64x8::splat(2.0) / f64x8::splat(3.0) * t14 * t86 * t88));
            let t99 = t5 * t19 * t68 / f64x8::splat(8.0);
            let t101 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t20 - t99));
            let t104 = f64x8::splat(2.0) * t29 * t80 * t30;
            let t105 = v_rho1 * v_rho1;
            let t106 = t13 * t105;
            let t107 = t79 * t6;
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t30 * t30;
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t108 * t110;
            let t115 = ((t28).select(f64x8::splat(0.0), -t104 - f64x8::splat(2.0) / f64x8::splat(3.0) * t106 * t111));
            let t121 = t5 * t34 * t68 / f64x8::splat(8.0);
            let t123 = ((t25).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t115 * t20 - t121));
            let t124 = t101 + t123;
            let t125 = t6 * t124;
            let t127 = t6 * t39;
            let t128 = f64x8::splat(1.0) / t52;
            let t129 = t128 * t40;
            let t130 = t59 * t61;
            let t135 = f64x8::splat(1.0) / t20 / t6;
            let t136 = t54 * t135;
            let t139 = t128 * t41;
            let t143 = f64x8::splat(1.0) / t47 / t6;
            let t147 = f64x8::splat(0.0011875159256848119) * t129 * t130 * t68 - f64x8::splat(3.4654072829174125) * t53 * t136 - f64x8::splat(3.4654072829174125) * t139 * t136 + f64x8::splat(648.4885723708404) * t66 * t67 * t143;
            let t148 = t72 * t147;
            let t150 = f64x8::splat(3.0) * t127 * t148;
            let tvrho0 = t125 * t75 - t150 + tzk0;
            acc_vrho_0 = tvrho0;
            let t151 = v_rho0 * v_rho0;
            let t152 = t13 * t151;
            let t153 = t108 * t85;
            let t157 = ((t10).select(f64x8::splat(0.0), -t83 - f64x8::splat(2.0) / f64x8::splat(3.0) * t152 * t153));
            let t162 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t157 * t20 - t99));
            let t165 = t7 * t110;
            let t167 = -v_rho1 * t80 + t7;
            let t172 = ((t28).select(f64x8::splat(0.0), f64x8::splat(2.0) * t76 * t30 - t104 + f64x8::splat(2.0) / f64x8::splat(3.0) * t29 * t165 * t167));
            let t177 = ((t25).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t172 * t20 - t121));
            let t178 = t162 + t177;
            let t179 = t6 * t178;
            let tvrho1 = t179 * t75 - t150 + tzk0;
            acc_vrho_1 = tvrho1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
