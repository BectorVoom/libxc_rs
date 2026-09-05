//! MGGA_C_BC95 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_bc95.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_bc95_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_copp: f64,
    param_css: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_copp = f64x8::splat(param_copp);
    let param_css = f64x8::splat(param_css);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t4);
            let t6 = ((t4).select(zeta_threshold, f64x8::splat(1.0)));
            let t7 = f64x8::splat(M_CBRT3);
            let t8 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t9 = (simd::cbrt(t8));
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = t11 * t11;
            let t13 = t10 * t12;
            let t14 = (simd::cbrt(v_rho));
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = f64x8::splat(M_CBRT2);
            let t18 = (simd::cbrt(zeta_threshold));
            let t20 = ((t4).select(f64x8::splat(1.0) / t18, f64x8::splat(1.0)));
            let t22 = t13 * t15 * t16 * t20;
            let t24 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t22;
            let t25 = ((t22).sqrt());
            let t28 = ((t22) * (t22).sqrt());
            let t30 = t7 * t7;
            let t31 = t9 * t9;
            let t32 = t30 * t31;
            let t33 = t32 * t11;
            let t34 = t14 * t14;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t16 * t16;
            let t38 = t20 * t20;
            let t40 = t33 * t35 * t36 * t38;
            let t42 = f64x8::splat(3.79785) * t25 + f64x8::splat(0.8969) * t22 + f64x8::splat(0.204775) * t28 + f64x8::splat(0.123235) * t40;
            let t45 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t42;
            let t46 = (simd::ln(t45));
            let t48 = f64x8::splat(0.0621814) * t24 * t46;
            let t50 = t18 * zeta_threshold;
            let t52 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t50, f64x8::splat(2.0) * t16));
            let t54 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t50, f64x8::splat(0.0)));
            let t58 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t16 - f64x8::splat(2.0));
            let t59 = (t52 + t54 - f64x8::splat(2.0)) * t58;
            let t61 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t22;
            let t66 = f64x8::splat(7.05945) * t25 + f64x8::splat(1.549425) * t22 + f64x8::splat(0.420775) * t28 + f64x8::splat(0.1562925) * t40;
            let t69 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t66;
            let t70 = (simd::ln(t69));
            let t74 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t22;
            let t79 = f64x8::splat(5.1785) * t25 + f64x8::splat(0.905775) * t22 + f64x8::splat(0.1100325) * t28 + f64x8::splat(0.1241775) * t40;
            let t82 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t79;
            let t83 = (simd::ln(t82));
            let t84 = t74 * t83;
            let t93 = ((t5).select(f64x8::splat(0.0), t6 * (-t48 + t59 * (-f64x8::splat(0.0310907) * t61 * t70 + t48 - f64x8::splat(0.0197516734986138) * t84) + f64x8::splat(0.0197516734986138) * t59 * t84) / f64x8::splat(2.0)));
            let t94 = t93 * v_tau;
            let t96 = f64x8::splat(1.0) / t34 / v_rho;
            let t97 = t36 * t96;
            let t99 = f64x8::splat(1.0) / v_rho;
            let t101 = f64x8::splat(1.0) / v_tau;
            let t104 = f64x8::splat(1.0) - v_sigma * t99 * t101 / f64x8::splat(8.0);
            let t105 = f64x8::splat(M_CBRT6);
            let t106 = t104 * t105;
            let t107 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t108 = (simd::cbrt(t107));
            let t109 = t108 * t108;
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = param_css * v_sigma;
            let t112 = v_rho * v_rho;
            let t114 = f64x8::splat(1.0) / t34 / t112;
            let t115 = t36 * t114;
            let t117 = t111 * t115 + f64x8::splat(1.0);
            let t118 = t117 * t117;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t110 * t119;
            let t121 = t106 * t120;
            let t123 = f64x8::splat(10.0) / f64x8::splat(9.0) * t94 * t97 * t121;
            let t125 = t10 * t12 * t15;
            let t127 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t125;
            let t128 = ((t125).sqrt());
            let t131 = ((t125) * (t125).sqrt());
            let t134 = t32 * t11 * t35;
            let t136 = f64x8::splat(3.79785) * t128 + f64x8::splat(0.8969) * t125 + f64x8::splat(0.204775) * t131 + f64x8::splat(0.123235) * t134;
            let t139 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t136;
            let t140 = (simd::ln(t139));
            let t143 = ((t4).select(t50, f64x8::splat(1.0)));
            let t146 = (f64x8::splat(2.0) * t143 - f64x8::splat(2.0)) * t58;
            let t148 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t125;
            let t153 = f64x8::splat(5.1785) * t128 + f64x8::splat(0.905775) * t125 + f64x8::splat(0.1100325) * t131 + f64x8::splat(0.1241775) * t134;
            let t156 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t153;
            let t157 = (simd::ln(t156));
            let t162 = -f64x8::splat(0.0621814) * t127 * t140 + f64x8::splat(0.0197516734986138) * t146 * t148 * t157 - f64x8::splat(2.0) * t93;
            let t166 = f64x8::splat(2.0) * param_copp * v_sigma * t115 + f64x8::splat(1.0);
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t162 * t167;
            let tzk0 = t123 + t168;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
