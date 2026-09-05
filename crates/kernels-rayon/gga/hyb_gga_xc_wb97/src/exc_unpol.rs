//! HYB_GGA_XC_WB97 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_xc_wb97.c`
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
pub fn hyb_gga_xc_wb97_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_3: f64,
    param_c_x_4: f64,
    param_c_x_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_x_1 = f64x8::splat(param_c_x_1);
    let param_c_x_2 = f64x8::splat(param_c_x_2);
    let param_c_x_3 = f64x8::splat(param_c_x_3);
    let param_c_x_4 = f64x8::splat(param_c_x_4);
    let param_c_x_0 = f64x8::splat(param_c_x_0);
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ab_1 = f64x8::splat(param_c_ab_1);
    let param_c_ab_2 = f64x8::splat(param_c_ab_2);
    let param_c_ab_3 = f64x8::splat(param_c_ab_3);
    let param_c_ab_4 = f64x8::splat(param_c_ab_4);
    let param_c_ab_0 = f64x8::splat(param_c_ab_0);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t3);
            let t5 = f64x8::splat(M_CBRT3);
            let t6 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t7 = (simd::cbrt(t6));
            let t8 = t5 * t7;
            let t9 = f64x8::splat(M_CBRT4);
            let t10 = t9 * t9;
            let t11 = f64x8::splat(M_CBRT2);
            let t13 = t8 * t10 * t11;
            let t14 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * zeta_threshold;
            let t18 = ((t14).select(t16, f64x8::splat(2.0) * t11));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(f64x8::splat(9.0)));
            let t22 = t21 * t21;
            let t23 = t7 * t7;
            let t25 = t22 * t23 * param_hyb_omega_0;
            let t26 = f64x8::splat(1.0) / t19;
            let t28 = ((t14).select(t15, t11));
            let t30 = t11 / t28;
            let t33 = t25 * t5 * t26 * t30 / f64x8::splat(18.0);
            let t34 = (f64x8::splat(1.35)).simd_le(t33);
            let t35 = (f64x8::splat(1.35)).simd_lt(t33);
            let t36 = ((t35).select(t33, f64x8::splat(1.35)));
            let t37 = t36 * t36;
            let t40 = t37 * t37;
            let t41 = f64x8::splat(1.0) / t40;
            let t43 = t40 * t37;
            let t44 = f64x8::splat(1.0) / t43;
            let t46 = t40 * t40;
            let t47 = f64x8::splat(1.0) / t46;
            let t50 = f64x8::splat(1.0) / t46 / t37;
            let t53 = f64x8::splat(1.0) / t46 / t40;
            let t56 = f64x8::splat(1.0) / t46 / t43;
            let t58 = t46 * t46;
            let t59 = f64x8::splat(1.0) / t58;
            let t62 = ((t35).select(f64x8::splat(1.35), t33));
            let t63 = ((f64x8::splat(M_PI)).sqrt());
            let t64 = f64x8::splat(1.0) / t62;
            let t66 = (simd::erf(t64 / f64x8::splat(2.0)));
            let t68 = t62 * t62;
            let t69 = f64x8::splat(1.0) / t68;
            let t71 = (simd::exp(-t69 / f64x8::splat(4.0)));
            let t72 = t71 - f64x8::splat(1.0);
            let t75 = t71 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t68 * t72;
            let t78 = f64x8::splat(2.0) * t62 * t75 + t63 * t66;
            let t82 = ((t34).select(f64x8::splat(1.0) / t37 / f64x8::splat(36.0) - t41 / f64x8::splat(960.0) + t44 / f64x8::splat(26880.0) - t47 / f64x8::splat(829440.0) + t50 / f64x8::splat(28385280.0) - t53 / f64x8::splat(1073479680.0) + t56 / f64x8::splat(44590694400.0) - t59 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t62 * t78));
            let t84 = param_c_x_1;
            let t85 = t84 * v_sigma;
            let t86 = t11 * t11;
            let t87 = v_rho * v_rho;
            let t88 = t19 * t19;
            let t90 = f64x8::splat(1.0) / t88 / t87;
            let t91 = t86 * t90;
            let t93 = v_sigma * t86 * t90;
            let t95 = f64x8::splat(1.0) + f64x8::splat(0.004) * t93;
            let t96 = f64x8::splat(1.0) / t95;
            let t100 = param_c_x_2;
            let t101 = v_sigma * v_sigma;
            let t102 = t100 * t101;
            let t103 = t87 * t87;
            let t104 = t103 * v_rho;
            let t106 = f64x8::splat(1.0) / t19 / t104;
            let t107 = t11 * t106;
            let t108 = t95 * t95;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t107 * t109;
            let t113 = param_c_x_3;
            let t114 = t101 * v_sigma;
            let t115 = t113 * t114;
            let t116 = t103 * t103;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t108 * t95;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t117 * t119;
            let t123 = param_c_x_4;
            let t124 = t101 * t101;
            let t125 = t123 * t124;
            let t126 = t116 * t87;
            let t128 = f64x8::splat(1.0) / t88 / t126;
            let t129 = t86 * t128;
            let t130 = t108 * t108;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t129 * t131;
            let t135 = param_c_x_0 + f64x8::splat(0.004) * t85 * t91 * t96 + f64x8::splat(3.2e-05) * t102 * t110 + f64x8::splat(2.56e-07) * t115 * t120 + f64x8::splat(1.024e-09) * t125 * t132;
            let t136 = t82 * t135;
            let t140 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t13 * t20 * t136));
            let t141 = f64x8::splat(2.0) * t140;
            let t142 = ((t3).select(zeta_threshold, f64x8::splat(1.0)));
            let t143 = t8 * t10;
            let t146 = ((t3).select(f64x8::splat(1.0) / t15, f64x8::splat(1.0)));
            let t148 = t143 * t26 * t11 * t146;
            let t150 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t148;
            let t151 = ((t148).sqrt());
            let t154 = ((t148) * (t148).sqrt());
            let t156 = t5 * t5;
            let t157 = t156 * t23;
            let t158 = t157 * t9;
            let t159 = f64x8::splat(1.0) / t88;
            let t161 = t146 * t146;
            let t163 = t158 * t159 * t86 * t161;
            let t165 = f64x8::splat(3.79785) * t151 + f64x8::splat(0.8969) * t148 + f64x8::splat(0.204775) * t154 + f64x8::splat(0.123235) * t163;
            let t168 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t165;
            let t169 = (simd::ln(t168));
            let t171 = f64x8::splat(0.062182) * t150 * t169;
            let t173 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t16, f64x8::splat(0.0)));
            let t177 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t11 - f64x8::splat(2.0));
            let t178 = (t18 + t173 - f64x8::splat(2.0)) * t177;
            let t180 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t148;
            let t185 = f64x8::splat(7.05945) * t151 + f64x8::splat(1.549425) * t148 + f64x8::splat(0.420775) * t154 + f64x8::splat(0.1562925) * t163;
            let t188 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t185;
            let t189 = (simd::ln(t188));
            let t193 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t148;
            let t198 = f64x8::splat(5.1785) * t151 + f64x8::splat(0.905775) * t148 + f64x8::splat(0.1100325) * t154 + f64x8::splat(0.1241775) * t163;
            let t201 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t198;
            let t202 = (simd::ln(t201));
            let t203 = t193 * t202;
            let t212 = ((t4).select(f64x8::splat(0.0), t142 * (-t171 + t178 * (-f64x8::splat(0.03109) * t180 * t189 + t171 - f64x8::splat(0.019751789702565206) * t203) + f64x8::splat(0.019751789702565206) * t178 * t203) / f64x8::splat(2.0)));
            let t214 = param_c_ss_1;
            let t215 = t214 * v_sigma;
            let t217 = f64x8::splat(1.0) + f64x8::splat(0.2) * t93;
            let t218 = f64x8::splat(1.0) / t217;
            let t222 = param_c_ss_2;
            let t223 = t222 * t101;
            let t224 = t217 * t217;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t107 * t225;
            let t229 = param_c_ss_3;
            let t230 = t229 * t114;
            let t231 = t224 * t217;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t117 * t232;
            let t236 = param_c_ss_4;
            let t237 = t236 * t124;
            let t238 = t224 * t224;
            let t239 = f64x8::splat(1.0) / t238;
            let t240 = t129 * t239;
            let t243 = param_c_ss_0 + f64x8::splat(0.2) * t215 * t91 * t218 + f64x8::splat(0.08) * t223 * t226 + f64x8::splat(0.032) * t230 * t233 + f64x8::splat(0.0064) * t237 * t240;
            let t245 = f64x8::splat(2.0) * t212 * t243;
            let t247 = t8 * t10 * t26;
            let t249 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t247;
            let t250 = ((t247).sqrt());
            let t253 = ((t247) * (t247).sqrt());
            let t256 = t157 * t9 * t159;
            let t258 = f64x8::splat(3.79785) * t250 + f64x8::splat(0.8969) * t247 + f64x8::splat(0.204775) * t253 + f64x8::splat(0.123235) * t256;
            let t261 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t258;
            let t262 = (simd::ln(t261));
            let t265 = ((t3).select(t16, f64x8::splat(1.0)));
            let t268 = (f64x8::splat(2.0) * t265 - f64x8::splat(2.0)) * t177;
            let t270 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t247;
            let t275 = f64x8::splat(5.1785) * t250 + f64x8::splat(0.905775) * t247 + f64x8::splat(0.1100325) * t253 + f64x8::splat(0.1241775) * t256;
            let t278 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t275;
            let t279 = (simd::ln(t278));
            let t284 = -f64x8::splat(0.062182) * t249 * t262 + f64x8::splat(0.019751789702565206) * t268 * t270 * t279 - f64x8::splat(2.0) * t212;
            let t286 = param_c_ab_1;
            let t287 = t286 * v_sigma;
            let t289 = f64x8::splat(1.0) + f64x8::splat(0.006) * t93;
            let t290 = f64x8::splat(1.0) / t289;
            let t294 = param_c_ab_2;
            let t295 = t294 * t101;
            let t296 = t289 * t289;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t107 * t297;
            let t301 = param_c_ab_3;
            let t302 = t301 * t114;
            let t303 = t296 * t289;
            let t304 = f64x8::splat(1.0) / t303;
            let t305 = t117 * t304;
            let t308 = param_c_ab_4;
            let t309 = t308 * t124;
            let t310 = t296 * t296;
            let t311 = f64x8::splat(1.0) / t310;
            let t312 = t129 * t311;
            let t315 = param_c_ab_0 + f64x8::splat(0.006) * t287 * t91 * t290 + f64x8::splat(7.2e-05) * t295 * t298 + f64x8::splat(8.64e-07) * t302 * t305 + f64x8::splat(5.184e-09) * t309 * t312;
            let t316 = t284 * t315;
            let tzk0 = t141 + t245 + t316;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
