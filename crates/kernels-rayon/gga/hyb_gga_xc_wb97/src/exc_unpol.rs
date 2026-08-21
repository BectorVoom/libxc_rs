//! HYB_GGA_XC_WB97 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/hyb_gga_xc_wb97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = M_CBRT3;
        let t6 = 1.0 / M_PI;
        let t7 = pow_1_3(t6);
        let t8 = t5 * t7;
        let t9 = M_CBRT4;
        let t10 = t9 * t9;
        let t11 = M_CBRT2;
        let t13 = t8 * t10 * t11;
        let t14 = 2.0 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * zeta_threshold;
        let t18 = piecewise3(t14, t16, 2.0 * t11);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = pow_1_3(9.0);
        let t22 = t21 * t21;
        let t23 = t7 * t7;
        let t25 = t22 * t23 * param_hyb_omega_0;
        let t26 = 1.0 / t19;
        let t28 = piecewise3(t14, t15, t11);
        let t30 = t11 / t28;
        let t33 = t25 * t5 * t26 * t30 / 18.0;
        let t34 = 1.35 <= t33;
        let t35 = 1.35 < t33;
        let t36 = piecewise3(t35, t33, 1.35);
        let t37 = t36 * t36;
        let t40 = t37 * t37;
        let t41 = 1.0 / t40;
        let t43 = t40 * t37;
        let t44 = 1.0 / t43;
        let t46 = t40 * t40;
        let t47 = 1.0 / t46;
        let t50 = 1.0 / t46 / t37;
        let t53 = 1.0 / t46 / t40;
        let t56 = 1.0 / t46 / t43;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t62 = piecewise3(t35, 1.35, t33);
        let t63 = rmath::sqrt(M_PI);
        let t64 = 1.0 / t62;
        let t66 = rmath::erf(t64 / 2.0);
        let t68 = t62 * t62;
        let t69 = 1.0 / t68;
        let t71 = rmath::exp(-t69 / 4.0);
        let t72 = t71 - 1.0;
        let t75 = t71 - 3.0 / 2.0 - 2.0 * t68 * t72;
        let t78 = 2.0 * t62 * t75 + t63 * t66;
        let t82 = piecewise3(t34, 1.0 / t37 / 36.0 - t41 / 960.0 + t44 / 26880.0 - t47 / 829440.0 + t50 / 28385280.0 - t53 / 1073479680.0 + t56 / 44590694400.0 - t59 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t62 * t78);
        let t84 = param_c_x_1;
        let t85 = t84 * sigma[ip];
        let t86 = t11 * t11;
        let t87 = rho[ip] * rho[ip];
        let t88 = t19 * t19;
        let t90 = 1.0 / t88 / t87;
        let t91 = t86 * t90;
        let t93 = sigma[ip] * t86 * t90;
        let t95 = 1.0 + 0.004 * t93;
        let t96 = 1.0 / t95;
        let t100 = param_c_x_2;
        let t101 = sigma[ip] * sigma[ip];
        let t102 = t100 * t101;
        let t103 = t87 * t87;
        let t104 = t103 * rho[ip];
        let t106 = 1.0 / t19 / t104;
        let t107 = t11 * t106;
        let t108 = t95 * t95;
        let t109 = 1.0 / t108;
        let t110 = t107 * t109;
        let t113 = param_c_x_3;
        let t114 = t101 * sigma[ip];
        let t115 = t113 * t114;
        let t116 = t103 * t103;
        let t117 = 1.0 / t116;
        let t118 = t108 * t95;
        let t119 = 1.0 / t118;
        let t120 = t117 * t119;
        let t123 = param_c_x_4;
        let t124 = t101 * t101;
        let t125 = t123 * t124;
        let t126 = t116 * t87;
        let t128 = 1.0 / t88 / t126;
        let t129 = t86 * t128;
        let t130 = t108 * t108;
        let t131 = 1.0 / t130;
        let t132 = t129 * t131;
        let t135 = param_c_x_0 + 0.004 * t85 * t91 * t96 + 3.2e-05 * t102 * t110 + 2.56e-07 * t115 * t120 + 1.024e-09 * t125 * t132;
        let t136 = t82 * t135;
        let t140 = piecewise3(t4, 0.0, -3.0 / 64.0 * t13 * t20 * t136);
        let t141 = 2.0 * t140;
        let t142 = piecewise3(t3, zeta_threshold, 1.0);
        let t143 = t8 * t10;
        let t146 = piecewise3(t3, 1.0 / t15, 1.0);
        let t148 = t143 * t26 * t11 * t146;
        let t150 = 1.0 + 0.053425 * t148;
        let t151 = rmath::sqrt(t148);
        let t154 = pow_3_2(t148);
        let t156 = t5 * t5;
        let t157 = t156 * t23;
        let t158 = t157 * t9;
        let t159 = 1.0 / t88;
        let t161 = t146 * t146;
        let t163 = t158 * t159 * t86 * t161;
        let t165 = 3.79785 * t151 + 0.8969 * t148 + 0.204775 * t154 + 0.123235 * t163;
        let t168 = 1.0 + 16.081824322151103 / t165;
        let t169 = rmath::ln(t168);
        let t171 = 0.062182 * t150 * t169;
        let t173 = piecewise3(0.0 <= zeta_threshold, t16, 0.0);
        let t177 = 1.0 / (2.0 * t11 - 2.0);
        let t178 = (t18 + t173 - 2.0) * t177;
        let t180 = 1.0 + 0.05137 * t148;
        let t185 = 7.05945 * t151 + 1.549425 * t148 + 0.420775 * t154 + 0.1562925 * t163;
        let t188 = 1.0 + 32.1646831778707 / t185;
        let t189 = rmath::ln(t188);
        let t193 = 1.0 + 0.0278125 * t148;
        let t198 = 5.1785 * t151 + 0.905775 * t148 + 0.1100325 * t154 + 0.1241775 * t163;
        let t201 = 1.0 + 29.608574643216677 / t198;
        let t202 = rmath::ln(t201);
        let t203 = t193 * t202;
        let t212 = piecewise3(t4, 0.0, t142 * (-t171 + t178 * (-0.03109 * t180 * t189 + t171 - 0.019751789702565206 * t203) + 0.019751789702565206 * t178 * t203) / 2.0);
        let t214 = param_c_ss_1;
        let t215 = t214 * sigma[ip];
        let t217 = 1.0 + 0.2 * t93;
        let t218 = 1.0 / t217;
        let t222 = param_c_ss_2;
        let t223 = t222 * t101;
        let t224 = t217 * t217;
        let t225 = 1.0 / t224;
        let t226 = t107 * t225;
        let t229 = param_c_ss_3;
        let t230 = t229 * t114;
        let t231 = t224 * t217;
        let t232 = 1.0 / t231;
        let t233 = t117 * t232;
        let t236 = param_c_ss_4;
        let t237 = t236 * t124;
        let t238 = t224 * t224;
        let t239 = 1.0 / t238;
        let t240 = t129 * t239;
        let t243 = param_c_ss_0 + 0.2 * t215 * t91 * t218 + 0.08 * t223 * t226 + 0.032 * t230 * t233 + 0.0064 * t237 * t240;
        let t245 = 2.0 * t212 * t243;
        let t247 = t8 * t10 * t26;
        let t249 = 1.0 + 0.053425 * t247;
        let t250 = rmath::sqrt(t247);
        let t253 = pow_3_2(t247);
        let t256 = t157 * t9 * t159;
        let t258 = 3.79785 * t250 + 0.8969 * t247 + 0.204775 * t253 + 0.123235 * t256;
        let t261 = 1.0 + 16.081824322151103 / t258;
        let t262 = rmath::ln(t261);
        let t265 = piecewise3(t3, t16, 1.0);
        let t268 = (2.0 * t265 - 2.0) * t177;
        let t270 = 1.0 + 0.0278125 * t247;
        let t275 = 5.1785 * t250 + 0.905775 * t247 + 0.1100325 * t253 + 0.1241775 * t256;
        let t278 = 1.0 + 29.608574643216677 / t275;
        let t279 = rmath::ln(t278);
        let t284 = -0.062182 * t249 * t262 + 0.019751789702565206 * t268 * t270 * t279 - 2.0 * t212;
        let t286 = param_c_ab_1;
        let t287 = t286 * sigma[ip];
        let t289 = 1.0 + 0.006 * t93;
        let t290 = 1.0 / t289;
        let t294 = param_c_ab_2;
        let t295 = t294 * t101;
        let t296 = t289 * t289;
        let t297 = 1.0 / t296;
        let t298 = t107 * t297;
        let t301 = param_c_ab_3;
        let t302 = t301 * t114;
        let t303 = t296 * t289;
        let t304 = 1.0 / t303;
        let t305 = t117 * t304;
        let t308 = param_c_ab_4;
        let t309 = t308 * t124;
        let t310 = t296 * t296;
        let t311 = 1.0 / t310;
        let t312 = t129 * t311;
        let t315 = param_c_ab_0 + 0.006 * t287 * t91 * t290 + 7.2e-05 * t295 * t298 + 8.64e-07 * t302 * t305 + 5.184e-09 * t309 * t312;
        let t316 = t284 * t315;
        let tzk0 = t141 + t245 + t316;
        zk[ip] += tzk0;
    }
}
