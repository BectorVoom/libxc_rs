//! GGA_C_BMK exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_bmk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_bmk_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_c_ab_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
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
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = 1.0 + t5;
        let t7 = t6 <= zeta_threshold;
        let t8 = rho0 <= dens_threshold || t7;
        let t9 = piecewise3(t7, zeta_threshold, t6);
        let t10 = M_CBRT3;
        let t11 = 1.0 / M_PI;
        let t12 = pow_1_3(t11);
        let t13 = t10 * t12;
        let t14 = M_CBRT4;
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = pow_1_3(t3);
        let t18 = 1.0 / t17;
        let t19 = M_CBRT2;
        let t20 = t18 * t19;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = 1.0 / t21;
        let t23 = pow_1_3(t6);
        let t25 = piecewise3(t7, t22, 1.0 / t23);
        let t27 = t16 * t20 * t25;
        let t29 = 1.0 + 0.53425e-1 * t27;
        let t30 = f64::sqrt(t27);
        let t33 = pow_3_2(t27);
        let t35 = t10 * t10;
        let t36 = t12 * t12;
        let t37 = t35 * t36;
        let t38 = t37 * t14;
        let t39 = t17 * t17;
        let t40 = 1.0 / t39;
        let t41 = t19 * t19;
        let t42 = t40 * t41;
        let t43 = t25 * t25;
        let t45 = t38 * t42 * t43;
        let t47 = 0.379785e1 * t30 + 0.8969e0 * t27 + 0.204775e0 * t33 + 0.123235e0 * t45;
        let t50 = 1.0 + 0.16081824322151104822e2 / t47;
        let t51 = f64::ln(t50);
        let t53 = 0.62182e-1 * t29 * t51;
        let t55 = t21 * zeta_threshold;
        let t57 = piecewise3(2.0 <= zeta_threshold, t55, 2.0 * t19);
        let t59 = piecewise3(0.0 <= zeta_threshold, t55, 0.0);
        let t63 = 1.0 / (2.0 * t19 - 2.0);
        let t64 = (t57 + t59 - 2.0) * t63;
        let t66 = 1.0 + 0.5137e-1 * t27;
        let t71 = 0.705945e1 * t30 + 0.1549425e1 * t27 + 0.420775e0 * t33 + 0.1562925e0 * t45;
        let t74 = 1.0 + 0.32164683177870697974e2 / t71;
        let t75 = f64::ln(t74);
        let t79 = 1.0 + 0.278125e-1 * t27;
        let t84 = 0.51785e1 * t30 + 0.905775e0 * t27 + 0.1100325e0 * t33 + 0.1241775e0 * t45;
        let t87 = 1.0 + 0.29608574643216675549e2 / t84;
        let t88 = f64::ln(t87);
        let t89 = t79 * t88;
        let t95 = -t53 + t64 * (-0.3109e-1 * t66 * t75 + t53 - 0.19751789702565206229e-1 * t89) + 0.19751789702565206229e-1 * t64 * t89;
        let t98 = piecewise3(t8, 0.0, t9 * t95 / 2.0);
        let t99 = param_c_ss_0;
        let t100 = param_c_ss_1;
        let t101 = t100 * sigma0;
        let t102 = rho0 * rho0;
        let t103 = pow_1_3(rho0);
        let t104 = t103 * t103;
        let t106 = 1.0 / t104 / t102;
        let t107 = sigma0 * t106;
        let t109 = 1.0 + 0.2e0 * t107;
        let t110 = 1.0 / t109;
        let t114 = param_c_ss_2;
        let t115 = sigma0 * sigma0;
        let t116 = t114 * t115;
        let t117 = t102 * t102;
        let t118 = t117 * rho0;
        let t120 = 1.0 / t103 / t118;
        let t121 = t109 * t109;
        let t122 = 1.0 / t121;
        let t123 = t120 * t122;
        let t126 = param_c_ss_3;
        let t127 = t115 * sigma0;
        let t128 = t126 * t127;
        let t129 = t117 * t117;
        let t130 = 1.0 / t129;
        let t131 = t121 * t109;
        let t132 = 1.0 / t131;
        let t133 = t130 * t132;
        let t136 = param_c_ss_4;
        let t137 = t115 * t115;
        let t138 = t136 * t137;
        let t139 = t129 * t102;
        let t141 = 1.0 / t104 / t139;
        let t142 = t121 * t121;
        let t143 = 1.0 / t142;
        let t144 = t141 * t143;
        let t147 = t99 + 0.2e0 * t101 * t106 * t110 + 0.4e-1 * t116 * t123 + 0.8e-2 * t128 * t133 + 0.16e-2 * t138 * t144;
        let t148 = t98 * t147;
        let t150 = 1.0 - t5;
        let t151 = t150 <= zeta_threshold;
        let t152 = rho1 <= dens_threshold || t151;
        let t153 = piecewise3(t151, zeta_threshold, t150);
        let t154 = pow_1_3(t150);
        let t156 = piecewise3(t151, t22, 1.0 / t154);
        let t158 = t16 * t20 * t156;
        let t160 = 1.0 + 0.53425e-1 * t158;
        let t161 = f64::sqrt(t158);
        let t164 = pow_3_2(t158);
        let t166 = t156 * t156;
        let t168 = t38 * t42 * t166;
        let t170 = 0.379785e1 * t161 + 0.8969e0 * t158 + 0.204775e0 * t164 + 0.123235e0 * t168;
        let t173 = 1.0 + 0.16081824322151104822e2 / t170;
        let t174 = f64::ln(t173);
        let t176 = 0.62182e-1 * t160 * t174;
        let t178 = 1.0 + 0.5137e-1 * t158;
        let t183 = 0.705945e1 * t161 + 0.1549425e1 * t158 + 0.420775e0 * t164 + 0.1562925e0 * t168;
        let t186 = 1.0 + 0.32164683177870697974e2 / t183;
        let t187 = f64::ln(t186);
        let t191 = 1.0 + 0.278125e-1 * t158;
        let t196 = 0.51785e1 * t161 + 0.905775e0 * t158 + 0.1100325e0 * t164 + 0.1241775e0 * t168;
        let t199 = 1.0 + 0.29608574643216675549e2 / t196;
        let t200 = f64::ln(t199);
        let t201 = t191 * t200;
        let t207 = -t176 + t64 * (-0.3109e-1 * t178 * t187 + t176 - 0.19751789702565206229e-1 * t201) + 0.19751789702565206229e-1 * t64 * t201;
        let t210 = piecewise3(t152, 0.0, t153 * t207 / 2.0);
        let t211 = t100 * sigma2;
        let t212 = rho1 * rho1;
        let t213 = pow_1_3(rho1);
        let t214 = t213 * t213;
        let t216 = 1.0 / t214 / t212;
        let t217 = sigma2 * t216;
        let t219 = 1.0 + 0.2e0 * t217;
        let t220 = 1.0 / t219;
        let t224 = sigma2 * sigma2;
        let t225 = t114 * t224;
        let t226 = t212 * t212;
        let t227 = t226 * rho1;
        let t229 = 1.0 / t213 / t227;
        let t230 = t219 * t219;
        let t231 = 1.0 / t230;
        let t232 = t229 * t231;
        let t235 = t224 * sigma2;
        let t236 = t126 * t235;
        let t237 = t226 * t226;
        let t238 = 1.0 / t237;
        let t239 = t230 * t219;
        let t240 = 1.0 / t239;
        let t241 = t238 * t240;
        let t244 = t224 * t224;
        let t245 = t136 * t244;
        let t246 = t237 * t212;
        let t248 = 1.0 / t214 / t246;
        let t249 = t230 * t230;
        let t250 = 1.0 / t249;
        let t251 = t248 * t250;
        let t254 = t99 + 0.2e0 * t211 * t216 * t220 + 0.4e-1 * t225 * t232 + 0.8e-2 * t236 * t241 + 0.16e-2 * t245 * t251;
        let t255 = t210 * t254;
        let t257 = t13 * t15 * t18;
        let t259 = 1.0 + 0.53425e-1 * t257;
        let t260 = f64::sqrt(t257);
        let t263 = pow_3_2(t257);
        let t266 = t37 * t14 * t40;
        let t268 = 0.379785e1 * t260 + 0.8969e0 * t257 + 0.204775e0 * t263 + 0.123235e0 * t266;
        let t271 = 1.0 + 0.16081824322151104822e2 / t268;
        let t272 = f64::ln(t271);
        let t274 = 0.62182e-1 * t259 * t272;
        let t275 = t2 * t2;
        let t276 = t275 * t275;
        let t277 = t3 * t3;
        let t278 = t277 * t277;
        let t279 = 1.0 / t278;
        let t280 = t276 * t279;
        let t281 = t23 * t6;
        let t282 = piecewise3(t7, t55, t281);
        let t283 = t154 * t150;
        let t284 = piecewise3(t151, t55, t283);
        let t285 = t282 + t284 - 2.0;
        let t286 = t285 * t63;
        let t288 = 1.0 + 0.5137e-1 * t257;
        let t293 = 0.705945e1 * t260 + 0.1549425e1 * t257 + 0.420775e0 * t263 + 0.1562925e0 * t266;
        let t296 = 1.0 + 0.32164683177870697974e2 / t293;
        let t297 = f64::ln(t296);
        let t301 = 1.0 + 0.278125e-1 * t257;
        let t306 = 0.51785e1 * t260 + 0.905775e0 * t257 + 0.1100325e0 * t263 + 0.1241775e0 * t266;
        let t309 = 1.0 + 0.29608574643216675549e2 / t306;
        let t310 = f64::ln(t309);
        let t311 = t301 * t310;
        let t313 = -0.3109e-1 * t288 * t297 + t274 - 0.19751789702565206229e-1 * t311;
        let t314 = t286 * t313;
        let t318 = -t274 + t280 * t314 + 0.19751789702565206229e-1 * t286 * t311 - t98 - t210;
        let t320 = param_c_ab_1;
        let t321 = t107 + t217;
        let t322 = t320 * t321;
        let t325 = 1.0 + 0.3e-2 * t107 + 0.3e-2 * t217;
        let t326 = 1.0 / t325;
        let t329 = param_c_ab_2;
        let t330 = t321 * t321;
        let t331 = t329 * t330;
        let t332 = t325 * t325;
        let t333 = 1.0 / t332;
        let t336 = param_c_ab_3;
        let t337 = t330 * t321;
        let t338 = t336 * t337;
        let t339 = t332 * t325;
        let t340 = 1.0 / t339;
        let t343 = param_c_ab_4;
        let t344 = t330 * t330;
        let t345 = t343 * t344;
        let t346 = t332 * t332;
        let t347 = 1.0 / t346;
        let t350 = param_c_ab_0 + 0.3e-2 * t322 * t326 + 0.9e-5 * t331 * t333 + 0.27e-7 * t338 * t340 + 0.81e-10 * t345 * t347;
        let t351 = t318 * t350;
        let tzk0 = t148 + t255 + t351;
        zk[ip] += tzk0;
    }
}
