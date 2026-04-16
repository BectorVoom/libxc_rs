//! MGGA_X_M11_L exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m11_l.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_m11_l_exc_unpol(
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
    param_c_0: f64,
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_6: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_11: f64,
    param_d_0: f64,
    param_d_1: f64,
    param_d_2: f64,
    param_d_3: f64,
    param_d_4: f64,
    param_d_5: f64,
    param_d_6: f64,
    param_d_7: f64,
    param_d_8: f64,
    param_d_9: f64,
    param_d_10: f64,
    param_d_11: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t13, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = pow_1_3(9.0);
        let t22 = t21 * t21;
        let t24 = pow_1_3(1.0 / M_PI);
        let t25 = t24 * t24;
        let t27 = t22 * t25 * param_hyb_omega_0;
        let t30 = piecewise3(t13, t14, t16);
        let t31 = 1.0 / t30;
        let t34 = t27 * t4 / t19 * t31 / 18.0;
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
        let t84 = M_CBRT6;
        let t85 = M_PI * M_PI;
        let t86 = pow_1_3(t85);
        let t87 = t86 * t86;
        let t88 = 1.0 / t87;
        let t89 = t84 * t88;
        let t90 = M_CBRT2;
        let t91 = t90 * t90;
        let t92 = sigma[ip] * t91;
        let t93 = rho[ip] * rho[ip];
        let t94 = t19 * t19;
        let t96 = 1.0 / t94 / t93;
        let t98 = t89 * t92 * t96;
        let t100 = 0.804e0 + 0.914625e-2 * t98;
        let t103 = 0.1804e1 - 0.646416e0 / t100;
        let t105 = param_a_1;
        let t106 = t84 * t84;
        let t108 = 3.0 / 10.0 * t106 * t87;
        let t109 = tau[ip] * t91;
        let t111 = 1.0 / t94 / rho[ip];
        let t112 = t109 * t111;
        let t113 = t108 - t112;
        let t114 = t105 * t113;
        let t115 = t108 + t112;
        let t116 = 1.0 / t115;
        let t118 = param_a_2;
        let t119 = t113 * t113;
        let t120 = t118 * t119;
        let t121 = t115 * t115;
        let t122 = 1.0 / t121;
        let t124 = param_a_3;
        let t125 = t119 * t113;
        let t126 = t124 * t125;
        let t127 = t121 * t115;
        let t128 = 1.0 / t127;
        let t130 = param_a_4;
        let t131 = t119 * t119;
        let t132 = t130 * t131;
        let t133 = t121 * t121;
        let t134 = 1.0 / t133;
        let t136 = param_a_5;
        let t137 = t131 * t113;
        let t138 = t136 * t137;
        let t139 = t133 * t115;
        let t140 = 1.0 / t139;
        let t142 = param_a_6;
        let t143 = t131 * t119;
        let t144 = t142 * t143;
        let t145 = t133 * t121;
        let t146 = 1.0 / t145;
        let t148 = param_a_7;
        let t149 = t131 * t125;
        let t150 = t148 * t149;
        let t151 = t133 * t127;
        let t152 = 1.0 / t151;
        let t154 = param_a_8;
        let t155 = t131 * t131;
        let t156 = t154 * t155;
        let t157 = t133 * t133;
        let t158 = 1.0 / t157;
        let t160 = param_a_9;
        let t161 = t155 * t113;
        let t162 = t160 * t161;
        let t164 = 1.0 / t157 / t115;
        let t166 = param_a_10;
        let t167 = t155 * t119;
        let t168 = t166 * t167;
        let t170 = 1.0 / t157 / t121;
        let t172 = param_a_11;
        let t173 = t155 * t125;
        let t174 = t172 * t173;
        let t176 = 1.0 / t157 / t127;
        let t178 = t114 * t116 + t120 * t122 + t126 * t128 + t132 * t134 + t138 * t140 + t144 * t146 + t150 * t152 + t156 * t158 + t162 * t164 + t168 * t170 + t174 * t176 + param_a_0;
        let t181 = f64::exp(-0.93189002206715572255e-2 * t98);
        let t183 = 0.1552e1 - 0.552e0 * t181;
        let t185 = param_b_1;
        let t186 = t185 * t113;
        let t188 = param_b_2;
        let t189 = t188 * t119;
        let t191 = param_b_3;
        let t192 = t191 * t125;
        let t194 = param_b_4;
        let t195 = t194 * t131;
        let t197 = param_b_5;
        let t198 = t197 * t137;
        let t200 = param_b_6;
        let t201 = t200 * t143;
        let t203 = param_b_7;
        let t204 = t203 * t149;
        let t206 = param_b_8;
        let t207 = t206 * t155;
        let t209 = param_b_9;
        let t210 = t209 * t161;
        let t212 = param_b_10;
        let t213 = t212 * t167;
        let t215 = param_b_11;
        let t216 = t215 * t173;
        let t218 = t186 * t116 + t189 * t122 + t192 * t128 + t195 * t134 + t198 * t140 + t201 * t146 + t204 * t152 + t207 * t158 + t210 * t164 + t213 * t170 + t216 * t176 + param_b_0;
        let t220 = t103 * t178 + t183 * t218;
        let t222 = 1.0 - t83;
        let t224 = param_c_1;
        let t225 = t224 * t113;
        let t227 = param_c_2;
        let t228 = t227 * t119;
        let t230 = param_c_3;
        let t231 = t230 * t125;
        let t233 = param_c_4;
        let t234 = t233 * t131;
        let t236 = param_c_5;
        let t237 = t236 * t137;
        let t239 = param_c_6;
        let t240 = t239 * t143;
        let t242 = param_c_7;
        let t243 = t242 * t149;
        let t245 = param_c_8;
        let t246 = t245 * t155;
        let t248 = param_c_9;
        let t249 = t248 * t161;
        let t251 = param_c_10;
        let t252 = t251 * t167;
        let t254 = param_c_11;
        let t255 = t254 * t173;
        let t257 = t225 * t116 + t228 * t122 + t231 * t128 + t234 * t134 + t237 * t140 + t240 * t146 + t243 * t152 + t246 * t158 + t249 * t164 + t252 * t170 + t255 * t176 + param_c_0;
        let t260 = param_d_1;
        let t261 = t260 * t113;
        let t263 = param_d_2;
        let t264 = t263 * t119;
        let t266 = param_d_3;
        let t267 = t266 * t125;
        let t269 = param_d_4;
        let t270 = t269 * t131;
        let t272 = param_d_5;
        let t273 = t272 * t137;
        let t275 = param_d_6;
        let t276 = t275 * t143;
        let t278 = param_d_7;
        let t279 = t278 * t149;
        let t281 = param_d_8;
        let t282 = t281 * t155;
        let t284 = param_d_9;
        let t285 = t284 * t161;
        let t287 = param_d_10;
        let t288 = t287 * t167;
        let t290 = param_d_11;
        let t291 = t290 * t173;
        let t293 = t261 * t116 + t264 * t122 + t267 * t128 + t270 * t134 + t273 * t140 + t276 * t146 + t279 * t152 + t282 * t158 + t285 * t164 + t288 * t170 + t291 * t176 + param_d_0;
        let t295 = t103 * t257 + t183 * t293;
        let t297 = t83 * t220 + t222 * t295;
        let t301 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t297);
        let tzk0 = 2.0 * t301;
        zk[ip] += tzk0;
    }
}
