//! MGGA_C_VSXC exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_vsxc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_vsxc_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha_ab: f64,
    param_alpha_ss: f64,
    param_dab_0: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    param_dss_0: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t3 = rho0 - rho1;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = 1.0 + t6;
        let t8 = t7 <= zeta_threshold;
        let t9 = rho0 <= dens_threshold || t8;
        let t10 = piecewise3::<f64>(t8, zeta_threshold, t7);
        let t11 = M_CBRT3;
        let t12 = 1.0 / M_PI;
        let t13 = pow_1_3::<f64>(t12);
        let t14 = t11 * t13;
        let t15 = M_CBRT4;
        let t16 = t15 * t15;
        let t17 = t14 * t16;
        let t18 = pow_1_3::<f64>(t4);
        let t19 = 1.0 / t18;
        let t20 = M_CBRT2;
        let t21 = t19 * t20;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = 1.0 / t22;
        let t24 = pow_1_3::<f64>(t7);
        let t26 = piecewise3::<f64>(t8, t23, 1.0 / t24);
        let t28 = t17 * t21 * t26;
        let t30 = 1.0 + 0.53425e-1 * t28;
        let t31 = f64::sqrt(t28);
        let t34 = pow_3_2::<f64>(t28);
        let t36 = t11 * t11;
        let t37 = t13 * t13;
        let t38 = t36 * t37;
        let t39 = t38 * t15;
        let t40 = t18 * t18;
        let t41 = 1.0 / t40;
        let t42 = t20 * t20;
        let t43 = t41 * t42;
        let t44 = t26 * t26;
        let t46 = t39 * t43 * t44;
        let t48 = 0.379785e1 * t31 + 0.8969e0 * t28 + 0.204775e0 * t34 + 0.123235e0 * t46;
        let t51 = 1.0 + 0.16081979498692535067e2 / t48;
        let t52 = f64::ln(t51);
        let t54 = 0.621814e-1 * t30 * t52;
        let t56 = t22 * zeta_threshold;
        let t58 = piecewise3::<f64>(2.0 <= zeta_threshold, t56, 2.0 * t20);
        let t60 = piecewise3::<f64>(0.0 <= zeta_threshold, t56, 0.0);
        let t64 = 1.0 / (2.0 * t20 - 2.0);
        let t65 = (t58 + t60 - 2.0) * t64;
        let t67 = 1.0 + 0.5137e-1 * t28;
        let t72 = 0.705945e1 * t31 + 0.1549425e1 * t28 + 0.420775e0 * t34 + 0.1562925e0 * t46;
        let t75 = 1.0 + 0.32163958997385070134e2 / t72;
        let t76 = f64::ln(t75);
        let t80 = 1.0 + 0.278125e-1 * t28;
        let t85 = 0.51785e1 * t31 + 0.905775e0 * t28 + 0.1100325e0 * t34 + 0.1241775e0 * t46;
        let t88 = 1.0 + 0.29608749977793437516e2 / t85;
        let t89 = f64::ln(t88);
        let t90 = t80 * t89;
        let t96 = -t54 + t65 * (-0.310907e-1 * t67 * t76 + t54 - 0.19751673498613801407e-1 * t90) + 0.19751673498613801407e-1 * t65 * t90;
        let t99 = piecewise3::<f64>(t9, 0.0, t10 * t96 / 2.0);
        let t100 = param_dss_0;
        let t101 = rho0 * rho0;
        let t102 = pow_1_3::<f64>(rho0);
        let t103 = t102 * t102;
        let t105 = 1.0 / t103 / t101;
        let t106 = sigma0 * t105;
        let t108 = 1.0 / t103 / rho0;
        let t110 = 2.0 * tau0 * t108;
        let t111 = M_CBRT6;
        let t112 = t111 * t111;
        let t113 = M_PI * M_PI;
        let t114 = pow_1_3::<f64>(t113);
        let t115 = t114 * t114;
        let t116 = t112 * t115;
        let t117 = 3.0 / 5.0 * t116;
        let t120 = 1.0 + param_alpha_ss * (t106 + t110 - t117);
        let t123 = param_dss_1;
        let t124 = t123 * sigma0;
        let t126 = param_dss_2;
        let t127 = t110 - t117;
        let t129 = t124 * t105 + t126 * t127;
        let t130 = t120 * t120;
        let t131 = 1.0 / t130;
        let t133 = param_dss_3;
        let t134 = sigma0 * sigma0;
        let t135 = t133 * t134;
        let t136 = t101 * t101;
        let t137 = t136 * rho0;
        let t139 = 1.0 / t102 / t137;
        let t141 = param_dss_4;
        let t142 = t141 * sigma0;
        let t145 = param_dss_5;
        let t146 = t127 * t127;
        let t148 = t142 * t105 * t127 + t135 * t139 + t145 * t146;
        let t149 = t130 * t120;
        let t150 = 1.0 / t149;
        let t152 = t100 / t120 + t129 * t131 + t148 * t150;
        let t153 = t99 * t152;
        let t154 = 1.0 / rho0;
        let t155 = sigma0 * t154;
        let t156 = 1.0 / tau0;
        let t159 = 1.0 - t155 * t156 / 8.0;
        let t160 = t153 * t159;
        let t162 = 1.0 - t6;
        let t163 = t162 <= zeta_threshold;
        let t164 = rho1 <= dens_threshold || t163;
        let t165 = piecewise3::<f64>(t163, zeta_threshold, t162);
        let t166 = pow_1_3::<f64>(t162);
        let t168 = piecewise3::<f64>(t163, t23, 1.0 / t166);
        let t170 = t17 * t21 * t168;
        let t172 = 1.0 + 0.53425e-1 * t170;
        let t173 = f64::sqrt(t170);
        let t176 = pow_3_2::<f64>(t170);
        let t178 = t168 * t168;
        let t180 = t39 * t43 * t178;
        let t182 = 0.379785e1 * t173 + 0.8969e0 * t170 + 0.204775e0 * t176 + 0.123235e0 * t180;
        let t185 = 1.0 + 0.16081979498692535067e2 / t182;
        let t186 = f64::ln(t185);
        let t188 = 0.621814e-1 * t172 * t186;
        let t190 = 1.0 + 0.5137e-1 * t170;
        let t195 = 0.705945e1 * t173 + 0.1549425e1 * t170 + 0.420775e0 * t176 + 0.1562925e0 * t180;
        let t198 = 1.0 + 0.32163958997385070134e2 / t195;
        let t199 = f64::ln(t198);
        let t203 = 1.0 + 0.278125e-1 * t170;
        let t208 = 0.51785e1 * t173 + 0.905775e0 * t170 + 0.1100325e0 * t176 + 0.1241775e0 * t180;
        let t211 = 1.0 + 0.29608749977793437516e2 / t208;
        let t212 = f64::ln(t211);
        let t213 = t203 * t212;
        let t219 = -t188 + t65 * (-0.310907e-1 * t190 * t199 + t188 - 0.19751673498613801407e-1 * t213) + 0.19751673498613801407e-1 * t65 * t213;
        let t222 = piecewise3::<f64>(t164, 0.0, t165 * t219 / 2.0);
        let t223 = rho1 * rho1;
        let t224 = pow_1_3::<f64>(rho1);
        let t225 = t224 * t224;
        let t227 = 1.0 / t225 / t223;
        let t228 = sigma2 * t227;
        let t230 = 1.0 / t225 / rho1;
        let t232 = 2.0 * tau1 * t230;
        let t235 = 1.0 + param_alpha_ss * (t228 + t232 - t117);
        let t238 = t123 * sigma2;
        let t240 = t232 - t117;
        let t242 = t126 * t240 + t238 * t227;
        let t243 = t235 * t235;
        let t244 = 1.0 / t243;
        let t246 = sigma2 * sigma2;
        let t247 = t133 * t246;
        let t248 = t223 * t223;
        let t249 = t248 * rho1;
        let t251 = 1.0 / t224 / t249;
        let t253 = t141 * sigma2;
        let t256 = t240 * t240;
        let t258 = t253 * t227 * t240 + t145 * t256 + t247 * t251;
        let t259 = t243 * t235;
        let t260 = 1.0 / t259;
        let t262 = t100 / t235 + t242 * t244 + t258 * t260;
        let t263 = t222 * t262;
        let t264 = 1.0 / rho1;
        let t265 = sigma2 * t264;
        let t266 = 1.0 / tau1;
        let t269 = 1.0 - t265 * t266 / 8.0;
        let t270 = t263 * t269;
        let t272 = t14 * t16 * t19;
        let t274 = 1.0 + 0.53425e-1 * t272;
        let t275 = f64::sqrt(t272);
        let t278 = pow_3_2::<f64>(t272);
        let t281 = t38 * t15 * t41;
        let t283 = 0.379785e1 * t275 + 0.8969e0 * t272 + 0.204775e0 * t278 + 0.123235e0 * t281;
        let t286 = 1.0 + 0.16081979498692535067e2 / t283;
        let t287 = f64::ln(t286);
        let t289 = 0.621814e-1 * t274 * t287;
        let t290 = t3 * t3;
        let t291 = t290 * t290;
        let t292 = t4 * t4;
        let t293 = t292 * t292;
        let t294 = 1.0 / t293;
        let t295 = t291 * t294;
        let t296 = t24 * t7;
        let t297 = piecewise3::<f64>(t8, t56, t296);
        let t298 = t166 * t162;
        let t299 = piecewise3::<f64>(t163, t56, t298);
        let t300 = t297 + t299 - 2.0;
        let t301 = t300 * t64;
        let t303 = 1.0 + 0.5137e-1 * t272;
        let t308 = 0.705945e1 * t275 + 0.1549425e1 * t272 + 0.420775e0 * t278 + 0.1562925e0 * t281;
        let t311 = 1.0 + 0.32163958997385070134e2 / t308;
        let t312 = f64::ln(t311);
        let t316 = 1.0 + 0.278125e-1 * t272;
        let t321 = 0.51785e1 * t275 + 0.905775e0 * t272 + 0.1100325e0 * t278 + 0.1241775e0 * t281;
        let t324 = 1.0 + 0.29608749977793437516e2 / t321;
        let t325 = f64::ln(t324);
        let t326 = t316 * t325;
        let t328 = -0.310907e-1 * t303 * t312 + t289 - 0.19751673498613801407e-1 * t326;
        let t329 = t301 * t328;
        let t333 = -t289 + t295 * t329 + 0.19751673498613801407e-1 * t301 * t326 - t99 - t222;
        let t334 = param_dab_0;
        let t335 = 6.0 / 5.0 * t116;
        let t338 = 1.0 + param_alpha_ab * (t106 + t228 + t110 + t232 - t335);
        let t341 = param_dab_1;
        let t342 = t106 + t228;
        let t344 = param_dab_2;
        let t345 = t110 + t232 - t335;
        let t347 = t341 * t342 + t344 * t345;
        let t348 = t338 * t338;
        let t349 = 1.0 / t348;
        let t351 = param_dab_3;
        let t352 = t342 * t342;
        let t354 = param_dab_4;
        let t355 = t354 * t342;
        let t357 = param_dab_5;
        let t358 = t345 * t345;
        let t360 = t355 * t345 + t351 * t352 + t357 * t358;
        let t361 = t348 * t338;
        let t362 = 1.0 / t361;
        let t364 = t334 / t338 + t347 * t349 + t360 * t362;
        let t365 = t333 * t364;
        let tzk0 = t160 + t270 + t365;
        zk[ip] += tzk0;
    }
}
