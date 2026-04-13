//! HYB_MGGA_XC_WB97MV exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_xc_wb97mv.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_mgga_xc_wb97mv_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c_os_0: f64,
    param_c_os_1: f64,
    param_c_os_2: f64,
    param_c_os_3: f64,
    param_c_os_4: f64,
    param_c_os_5: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_x_0: f64,
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t4 = 1.0 <= zeta_threshold;
        let t5 = rho[ip] / 2.0 <= dens_threshold || t4;
        let t6 = M_CBRT3;
        let t7 = 1.0 / M_PI;
        let t8 = pow_1_3(t7);
        let t9 = t6 * t8;
        let t10 = M_CBRT4;
        let t11 = t10 * t10;
        let t12 = M_CBRT2;
        let t14 = t9 * t11 * t12;
        let t15 = 2.0 <= zeta_threshold;
        let t16 = pow_1_3(zeta_threshold);
        let t17 = t16 * zeta_threshold;
        let t19 = piecewise3(t15, t17, 2.0 * t12);
        let t20 = pow_1_3(rho[ip]);
        let t21 = t19 * t20;
        let t22 = pow_1_3(9.0);
        let t23 = t22 * t22;
        let t24 = t8 * t8;
        let t26 = t23 * t24 * param_hyb_omega_0;
        let t27 = 1.0 / t20;
        let t29 = piecewise3(t15, t16, t12);
        let t31 = t12 / t29;
        let t34 = t26 * t6 * t27 * t31 / 18.0;
        let t35 = 0.135e1 <= t34;
        let t36 = 0.135e1 < t34;
        let t37 = piecewise3(t36, t34, 0.135e1);
        let t38 = t37 * t37;
        let t41 = t38 * t38;
        let t42 = 1.0 / t41;
        let t44 = t41 * t38;
        let t45 = 1.0 / t44;
        let t47 = t41 * t41;
        let t48 = 1.0 / t47;
        let t51 = 1.0 / t47 / t38;
        let t54 = 1.0 / t47 / t41;
        let t57 = 1.0 / t47 / t44;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t63 = piecewise3(t36, 0.135e1, t34);
        let t64 = f64::sqrt(M_PI);
        let t65 = 1.0 / t63;
        let t67 = erf_approx(t65 / 2.0);
        let t69 = t63 * t63;
        let t70 = 1.0 / t69;
        let t72 = f64::exp(-t70 / 4.0);
        let t73 = t72 - 1.0;
        let t76 = t72 - 3.0 / 2.0 - 2.0 * t69 * t73;
        let t79 = 2.0 * t63 * t76 + t64 * t67;
        let t83 = piecewise3(t35, 1.0 / t38 / 36.0 - t42 / 960.0 + t45 / 26880.0 - t48 / 829440.0 + t51 / 28385280.0 - t54 / 0.107347968e10 + t57 / 0.445906944e11 - t60 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t63 * t79);
        let t85 = param_c_x_1;
        let t86 = t85 * sigma[ip];
        let t87 = t12 * t12;
        let t88 = rho[ip] * rho[ip];
        let t89 = t20 * t20;
        let t91 = 1.0 / t89 / t88;
        let t92 = t87 * t91;
        let t93 = sigma[ip] * t87;
        let t94 = t93 * t91;
        let t96 = 1.0 + 0.4e-2 * t94;
        let t97 = 1.0 / t96;
        let t101 = param_c_x_2;
        let t102 = M_CBRT6;
        let t103 = t102 * t102;
        let t104 = M_PI * M_PI;
        let t105 = pow_1_3(t104);
        let t106 = t105 * t105;
        let t107 = t103 * t106;
        let t108 = 3.0 / 10.0 * t107;
        let t109 = tau[ip] * t87;
        let t111 = 1.0 / t89 / rho[ip];
        let t112 = t109 * t111;
        let t113 = t108 - t112;
        let t114 = t101 * t113;
        let t115 = t108 + t112;
        let t116 = 1.0 / t115;
        let t118 = param_c_x_0 + 0.4e-2 * t86 * t92 * t97 + t114 * t116;
        let t119 = t83 * t118;
        let t123 = piecewise3(t5, 0.0, -3.0 / 64.0 * t14 * t21 * t119);
        let t124 = 2.0 * t123;
        let t125 = piecewise3(t4, zeta_threshold, 1.0);
        let t126 = t9 * t11;
        let t129 = piecewise3(t4, 1.0 / t16, 1.0);
        let t131 = t126 * t27 * t12 * t129;
        let t133 = 1.0 + 0.53425e-1 * t131;
        let t134 = f64::sqrt(t131);
        let t137 = pow_3_2(t131);
        let t139 = t6 * t6;
        let t140 = t139 * t24;
        let t141 = t140 * t10;
        let t142 = 1.0 / t89;
        let t144 = t129 * t129;
        let t146 = t141 * t142 * t87 * t144;
        let t148 = 0.379785e1 * t134 + 0.8969e0 * t131 + 0.204775e0 * t137 + 0.123235e0 * t146;
        let t151 = 1.0 + 0.16081979498692535067e2 / t148;
        let t152 = f64::ln(t151);
        let t154 = 0.621814e-1 * t133 * t152;
        let t156 = piecewise3(0.0 <= zeta_threshold, t17, 0.0);
        let t160 = 1.0 / (2.0 * t12 - 2.0);
        let t161 = (t19 + t156 - 2.0) * t160;
        let t163 = 1.0 + 0.5137e-1 * t131;
        let t168 = 0.705945e1 * t134 + 0.1549425e1 * t131 + 0.420775e0 * t137 + 0.1562925e0 * t146;
        let t171 = 1.0 + 0.32163958997385070134e2 / t168;
        let t172 = f64::ln(t171);
        let t176 = 1.0 + 0.278125e-1 * t131;
        let t181 = 0.51785e1 * t134 + 0.905775e0 * t131 + 0.1100325e0 * t137 + 0.1241775e0 * t146;
        let t184 = 1.0 + 0.29608749977793437516e2 / t181;
        let t185 = f64::ln(t184);
        let t186 = t176 * t185;
        let t195 = piecewise3(t5, 0.0, t125 * (-t154 + t161 * (-0.310907e-1 * t163 * t172 + t154 - 0.19751673498613801407e-1 * t186) + 0.19751673498613801407e-1 * t161 * t186) / 2.0);
        let t197 = param_c_ss_1;
        let t198 = sigma[ip] * sigma[ip];
        let t199 = t198 * t198;
        let t200 = t197 * t199;
        let t201 = t88 * t88;
        let t202 = t201 * t201;
        let t203 = t202 * t88;
        let t205 = 1.0 / t89 / t203;
        let t208 = 1.0 + 0.2e0 * t94;
        let t209 = t208 * t208;
        let t210 = t209 * t209;
        let t211 = 1.0 / t210;
        let t212 = t87 * t205 * t211;
        let t215 = param_c_ss_2;
        let t216 = t215 * t113;
        let t218 = param_c_ss_3;
        let t219 = t113 * t113;
        let t220 = t218 * t219;
        let t221 = t115 * t115;
        let t222 = 1.0 / t221;
        let t224 = param_c_ss_4;
        let t225 = t219 * t219;
        let t226 = t224 * t225;
        let t227 = t221 * t221;
        let t228 = 1.0 / t227;
        let t229 = t226 * t228;
        let t230 = t198 * sigma[ip];
        let t231 = 1.0 / t202;
        let t233 = t209 * t208;
        let t234 = 1.0 / t233;
        let t238 = param_c_ss_0 + 0.64e-2 * t200 * t212 + t216 * t116 + t220 * t222 + 0.32e-1 * t229 * t230 * t231 * t234;
        let t240 = 2.0 * t195 * t238;
        let t242 = t9 * t11 * t27;
        let t244 = 1.0 + 0.53425e-1 * t242;
        let t245 = f64::sqrt(t242);
        let t248 = pow_3_2(t242);
        let t251 = t140 * t10 * t142;
        let t253 = 0.379785e1 * t245 + 0.8969e0 * t242 + 0.204775e0 * t248 + 0.123235e0 * t251;
        let t256 = 1.0 + 0.16081979498692535067e2 / t253;
        let t257 = f64::ln(t256);
        let t260 = piecewise3(t4, t17, 1.0);
        let t263 = (2.0 * t260 - 2.0) * t160;
        let t265 = 1.0 + 0.278125e-1 * t242;
        let t270 = 0.51785e1 * t245 + 0.905775e0 * t242 + 0.1100325e0 * t248 + 0.1241775e0 * t251;
        let t273 = 1.0 + 0.29608749977793437516e2 / t270;
        let t274 = f64::ln(t273);
        let t279 = -0.621814e-1 * t244 * t257 + 0.19751673498613801407e-1 * t263 * t265 * t274 - 2.0 * t195;
        let t281 = param_c_os_1;
        let t283 = 3.0 / 5.0 * t107 * t112;
        let t284 = tau[ip] * tau[ip];
        let t285 = t284 * t12;
        let t286 = t88 * rho[ip];
        let t288 = 1.0 / t20 / t286;
        let t290 = 4.0 * t285 * t288;
        let t291 = t283 - t290;
        let t292 = t281 * t291;
        let t293 = t283 + t290;
        let t294 = 1.0 / t293;
        let t296 = param_c_os_2;
        let t297 = t291 * t291;
        let t298 = t296 * t297;
        let t299 = t293 * t293;
        let t300 = 1.0 / t299;
        let t302 = param_c_os_3;
        let t303 = t302 * t297;
        let t304 = t303 * t300;
        let t306 = 1.0 + 0.6e-2 * t94;
        let t307 = 1.0 / t306;
        let t308 = t91 * t307;
        let t309 = t93 * t308;
        let t312 = param_c_os_4;
        let t313 = t297 * t297;
        let t314 = t313 * t297;
        let t315 = t312 * t314;
        let t316 = t299 * t299;
        let t318 = 1.0 / t316 / t299;
        let t320 = param_c_os_5;
        let t321 = t320 * t314;
        let t322 = t321 * t318;
        let t325 = param_c_os_0 + t292 * t294 + t298 * t300 + 0.6e-2 * t304 * t309 + t315 * t318 + 0.6e-2 * t322 * t309;
        let t326 = t279 * t325;
        let tzk0 = t124 + t240 + t326;
        zk[ip] += tzk0;
    }
}
