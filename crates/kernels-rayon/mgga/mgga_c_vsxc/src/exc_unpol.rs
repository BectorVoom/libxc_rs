//! MGGA_C_VSXC exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_vsxc.c`
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
pub fn mgga_c_vsxc_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_dss_0: f64,
    param_alpha_ss: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    param_dab_0: f64,
    param_alpha_ab: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_dss_0 = f64x8::splat(param_dss_0);
    let param_alpha_ss = f64x8::splat(param_alpha_ss);
    let param_dss_1 = f64x8::splat(param_dss_1);
    let param_dss_2 = f64x8::splat(param_dss_2);
    let param_dss_3 = f64x8::splat(param_dss_3);
    let param_dss_4 = f64x8::splat(param_dss_4);
    let param_dss_5 = f64x8::splat(param_dss_5);
    let param_dab_0 = f64x8::splat(param_dab_0);
    let param_alpha_ab = f64x8::splat(param_alpha_ab);
    let param_dab_1 = f64x8::splat(param_dab_1);
    let param_dab_2 = f64x8::splat(param_dab_2);
    let param_dab_3 = f64x8::splat(param_dab_3);
    let param_dab_4 = f64x8::splat(param_dab_4);
    let param_dab_5 = f64x8::splat(param_dab_5);
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
            let t94 = param_dss_0;
            let t95 = v_sigma * t36;
            let t96 = v_rho * v_rho;
            let t98 = f64x8::splat(1.0) / t34 / t96;
            let t99 = t95 * t98;
            let t100 = v_tau * t36;
            let t102 = f64x8::splat(1.0) / t34 / v_rho;
            let t103 = t100 * t102;
            let t104 = f64x8::splat(2.0) * t103;
            let t105 = f64x8::splat(M_CBRT6);
            let t106 = t105 * t105;
            let t107 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t108 = (simd::cbrt(t107));
            let t109 = t108 * t108;
            let t110 = t106 * t109;
            let t111 = f64x8::splat(3.0) / f64x8::splat(5.0) * t110;
            let t114 = f64x8::splat(1.0) + param_alpha_ss * (t99 + t104 - t111);
            let t117 = param_dss_1;
            let t118 = t117 * v_sigma;
            let t119 = t36 * t98;
            let t121 = param_dss_2;
            let t122 = t104 - t111;
            let t124 = t118 * t119 + t121 * t122;
            let t125 = t114 * t114;
            let t126 = f64x8::splat(1.0) / t125;
            let t128 = param_dss_3;
            let t129 = v_sigma * v_sigma;
            let t130 = t128 * t129;
            let t131 = t96 * t96;
            let t132 = t131 * v_rho;
            let t134 = f64x8::splat(1.0) / t14 / t132;
            let t135 = t16 * t134;
            let t138 = param_dss_4;
            let t139 = t138 * v_sigma;
            let t142 = param_dss_5;
            let t143 = t122 * t122;
            let t145 = t119 * t122 * t139 + f64x8::splat(2.0) * t130 * t135 + t142 * t143;
            let t146 = t125 * t114;
            let t147 = f64x8::splat(1.0) / t146;
            let t149 = t94 / t114 + t124 * t126 + t145 * t147;
            let t150 = t93 * t149;
            let t151 = f64x8::splat(1.0) / v_rho;
            let t152 = v_sigma * t151;
            let t153 = f64x8::splat(1.0) / v_tau;
            let t156 = f64x8::splat(1.0) - t152 * t153 / f64x8::splat(8.0);
            let t158 = f64x8::splat(2.0) * t150 * t156;
            let t160 = t10 * t12 * t15;
            let t162 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t160;
            let t163 = ((t160).sqrt());
            let t166 = ((t160) * (t160).sqrt());
            let t169 = t32 * t11 * t35;
            let t171 = f64x8::splat(3.79785) * t163 + f64x8::splat(0.8969) * t160 + f64x8::splat(0.204775) * t166 + f64x8::splat(0.123235) * t169;
            let t174 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t171;
            let t175 = (simd::ln(t174));
            let t178 = ((t4).select(t50, f64x8::splat(1.0)));
            let t181 = (f64x8::splat(2.0) * t178 - f64x8::splat(2.0)) * t58;
            let t183 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t160;
            let t188 = f64x8::splat(5.1785) * t163 + f64x8::splat(0.905775) * t160 + f64x8::splat(0.1100325) * t166 + f64x8::splat(0.1241775) * t169;
            let t191 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t188;
            let t192 = (simd::ln(t191));
            let t197 = -f64x8::splat(0.0621814) * t162 * t175 + f64x8::splat(0.0197516734986138) * t181 * t183 * t192 - f64x8::splat(2.0) * t93;
            let t198 = param_dab_0;
            let t200 = f64x8::splat(4.0) * t103;
            let t201 = f64x8::splat(6.0) / f64x8::splat(5.0) * t110;
            let t204 = f64x8::splat(1.0) + param_alpha_ab * (f64x8::splat(2.0) * t99 + t200 - t201);
            let t207 = param_dab_1;
            let t208 = t207 * v_sigma;
            let t211 = param_dab_2;
            let t212 = t200 - t201;
            let t214 = f64x8::splat(2.0) * t119 * t208 + t211 * t212;
            let t215 = t204 * t204;
            let t216 = f64x8::splat(1.0) / t215;
            let t218 = param_dab_3;
            let t219 = t218 * t129;
            let t222 = param_dab_4;
            let t223 = t222 * v_sigma;
            let t227 = param_dab_5;
            let t228 = t212 * t212;
            let t230 = f64x8::splat(2.0) * t119 * t212 * t223 + f64x8::splat(8.0) * t135 * t219 + t227 * t228;
            let t231 = t215 * t204;
            let t232 = f64x8::splat(1.0) / t231;
            let t234 = t198 / t204 + t214 * t216 + t230 * t232;
            let t235 = t197 * t234;
            let tzk0 = t158 + t235;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
