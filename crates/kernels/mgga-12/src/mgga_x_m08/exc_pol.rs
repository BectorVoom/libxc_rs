//! MGGA_X_M08 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 195 shared lines across all orders.
//! Delta: 195 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_m08_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
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
        // --- shared preamble (195 lines) ---
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = t34 * sigma0 * t39;
        let t43 = 0.804e0 + 0.914625e-2 * t41;
        let t46 = 0.1804e1 - 0.646416e0 / t43;
        let t47 = param_a_0;
        let t48 = param_a_1;
        let t49 = t29 * t29;
        let t51 = 3.0 / 10.0 * t49 * t32;
        let t53 = 1.0 / t37 / rho0;
        let t54 = tau0 * t53;
        let t55 = t51 - t54;
        let t56 = t48 * t55;
        let t57 = t51 + t54;
        let t58 = 1.0 / t57;
        let t60 = param_a_2;
        let t61 = t55 * t55;
        let t62 = t60 * t61;
        let t63 = t57 * t57;
        let t64 = 1.0 / t63;
        let t66 = param_a_3;
        let t67 = t61 * t55;
        let t68 = t66 * t67;
        let t69 = t63 * t57;
        let t70 = 1.0 / t69;
        let t72 = param_a_4;
        let t73 = t61 * t61;
        let t74 = t72 * t73;
        let t75 = t63 * t63;
        let t76 = 1.0 / t75;
        let t78 = param_a_5;
        let t79 = t73 * t55;
        let t80 = t78 * t79;
        let t81 = t75 * t57;
        let t82 = 1.0 / t81;
        let t84 = param_a_6;
        let t85 = t73 * t61;
        let t86 = t84 * t85;
        let t87 = t75 * t63;
        let t88 = 1.0 / t87;
        let t90 = param_a_7;
        let t91 = t73 * t67;
        let t92 = t90 * t91;
        let t93 = t75 * t69;
        let t94 = 1.0 / t93;
        let t96 = param_a_8;
        let t97 = t73 * t73;
        let t98 = t96 * t97;
        let t99 = t75 * t75;
        let t100 = 1.0 / t99;
        let t102 = param_a_9;
        let t103 = t97 * t55;
        let t104 = t102 * t103;
        let t106 = 1.0 / t99 / t57;
        let t108 = param_a_10;
        let t109 = t97 * t61;
        let t110 = t108 * t109;
        let t112 = 1.0 / t99 / t63;
        let t114 = param_a_11;
        let t115 = t97 * t67;
        let t116 = t114 * t115;
        let t118 = 1.0 / t99 / t69;
        let t120 = t98 * t100 + t104 * t106 + t110 * t112 + t116 * t118 + t56 * t58 + t62 * t64 + t68 * t70 + t74 * t76 + t80 * t82 + t86 * t88 + t92 * t94 + t47;
        let t123 = f64::exp(-0.93189002206715572255e-2 * t41);
        let t125 = 0.1552e1 - 0.552e0 * t123;
        let t126 = param_b_0;
        let t127 = param_b_1;
        let t128 = t127 * t55;
        let t130 = param_b_2;
        let t131 = t130 * t61;
        let t133 = param_b_3;
        let t134 = t133 * t67;
        let t136 = param_b_4;
        let t137 = t136 * t73;
        let t139 = param_b_5;
        let t140 = t139 * t79;
        let t142 = param_b_6;
        let t143 = t142 * t85;
        let t145 = param_b_7;
        let t146 = t145 * t91;
        let t148 = param_b_8;
        let t149 = t148 * t97;
        let t151 = param_b_9;
        let t152 = t151 * t103;
        let t154 = param_b_10;
        let t155 = t154 * t109;
        let t157 = param_b_11;
        let t158 = t157 * t115;
        let t160 = t149 * t100 + t152 * t106 + t155 * t112 + t158 * t118 + t128 * t58 + t131 * t64 + t134 * t70 + t137 * t76 + t140 * t82 + t143 * t88 + t146 * t94 + t126;
        let t162 = t46 * t120 + t125 * t160;
        let t166 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t162);
        let t167 = rho1 <= dens_threshold;
        let t168 = -t17;
        let t170 = piecewise5(t15, t12, t11, t16, t168 * t8);
        let t171 = 1.0 + t170;
        let t172 = t171 <= zeta_threshold;
        let t173 = pow_1_3(t171);
        let t175 = piecewise3(t172, t23, t173 * t171);
        let t176 = t175 * t27;
        let t177 = rho1 * rho1;
        let t178 = pow_1_3(rho1);
        let t179 = t178 * t178;
        let t181 = 1.0 / t179 / t177;
        let t183 = t34 * sigma2 * t181;
        let t185 = 0.804e0 + 0.914625e-2 * t183;
        let t188 = 0.1804e1 - 0.646416e0 / t185;
        let t190 = 1.0 / t179 / rho1;
        let t191 = tau1 * t190;
        let t192 = t51 - t191;
        let t193 = t48 * t192;
        let t194 = t51 + t191;
        let t195 = 1.0 / t194;
        let t197 = t192 * t192;
        let t198 = t60 * t197;
        let t199 = t194 * t194;
        let t200 = 1.0 / t199;
        let t202 = t197 * t192;
        let t203 = t66 * t202;
        let t204 = t199 * t194;
        let t205 = 1.0 / t204;
        let t207 = t197 * t197;
        let t208 = t72 * t207;
        let t209 = t199 * t199;
        let t210 = 1.0 / t209;
        let t212 = t207 * t192;
        let t213 = t78 * t212;
        let t214 = t209 * t194;
        let t215 = 1.0 / t214;
        let t217 = t207 * t197;
        let t218 = t84 * t217;
        let t219 = t209 * t199;
        let t220 = 1.0 / t219;
        let t222 = t207 * t202;
        let t223 = t90 * t222;
        let t224 = t209 * t204;
        let t225 = 1.0 / t224;
        let t227 = t207 * t207;
        let t228 = t96 * t227;
        let t229 = t209 * t209;
        let t230 = 1.0 / t229;
        let t232 = t227 * t192;
        let t233 = t102 * t232;
        let t235 = 1.0 / t229 / t194;
        let t237 = t227 * t197;
        let t238 = t108 * t237;
        let t240 = 1.0 / t229 / t199;
        let t242 = t227 * t202;
        let t243 = t114 * t242;
        let t245 = 1.0 / t229 / t204;
        let t247 = t193 * t195 + t198 * t200 + t203 * t205 + t208 * t210 + t213 * t215 + t218 * t220 + t223 * t225 + t228 * t230 + t233 * t235 + t238 * t240 + t243 * t245 + t47;
        let t250 = f64::exp(-0.93189002206715572255e-2 * t183);
        let t252 = 0.1552e1 - 0.552e0 * t250;
        let t253 = t127 * t192;
        let t255 = t130 * t197;
        let t257 = t133 * t202;
        let t259 = t136 * t207;
        let t261 = t139 * t212;
        let t263 = t142 * t217;
        let t265 = t145 * t222;
        let t267 = t148 * t227;
        let t269 = t151 * t232;
        let t271 = t154 * t237;
        let t273 = t157 * t242;
        let t275 = t253 * t195 + t255 * t200 + t257 * t205 + t259 * t210 + t261 * t215 + t263 * t220 + t265 * t225 + t267 * t230 + t269 * t235 + t271 * t240 + t273 * t245 + t126;
        let t277 = t188 * t247 + t252 * t275;
        let t281 = piecewise3(t167, 0.0, -3.0 / 8.0 * t6 * t176 * t277);
        let tzk0 = t166 + t281;
        zk[ip] += tzk0;
    }
}
