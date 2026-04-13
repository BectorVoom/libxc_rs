//! MGGA_X_M06L exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m06l.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_m06l_exc_pol(
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
    param_d_0: f64,
    param_d_1: f64,
    param_d_2: f64,
    param_d_3: f64,
    param_d_4: f64,
    param_d_5: f64,
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
        let t40 = sigma0 * t39;
        let t43 = 0.804e0 + 0.91464571985215458336e-2 * t34 * t40;
        let t46 = 0.1804e1 - 0.646416e0 / t43;
        let t47 = param_a_0;
        let t48 = param_a_1;
        let t49 = t29 * t29;
        let t50 = t49 * t32;
        let t51 = 3.0 / 10.0 * t50;
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
        let t116 = t114 * t97 * t67;
        let t118 = 1.0 / t99 / t69;
        let t120 = t98 * t100 + t104 * t106 + t110 * t112 + t116 * t118 + t56 * t58 + t62 * t64 + t68 * t70 + t74 * t76 + t80 * t82 + t86 * t88 + t92 * t94 + t47;
        let t122 = param_d_0;
        let t125 = 0.1120356e-2 * t50;
        let t126 = 1.0 + 0.186726e-2 * t40 + 0.373452e-2 * t54 - t125;
        let t129 = param_d_1;
        let t130 = t129 * sigma0;
        let t132 = param_d_2;
        let t134 = 3.0 / 5.0 * t50;
        let t135 = 2.0 * t54 - t134;
        let t137 = t130 * t39 + t132 * t135;
        let t138 = t126 * t126;
        let t139 = 1.0 / t138;
        let t141 = param_d_3;
        let t142 = sigma0 * sigma0;
        let t143 = t141 * t142;
        let t144 = t35 * t35;
        let t145 = t144 * rho0;
        let t147 = 1.0 / t36 / t145;
        let t149 = param_d_4;
        let t150 = t149 * sigma0;
        let t153 = param_d_5;
        let t154 = t135 * t135;
        let t156 = t150 * t39 * t135 + t143 * t147 + t153 * t154;
        let t157 = t138 * t126;
        let t158 = 1.0 / t157;
        let t160 = t46 * t120 + t122 / t126 + t137 * t139 + t156 * t158;
        let t164 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t160);
        let t165 = rho1 <= dens_threshold;
        let t166 = -t17;
        let t168 = piecewise5(t15, t12, t11, t16, t166 * t8);
        let t169 = 1.0 + t168;
        let t170 = t169 <= zeta_threshold;
        let t171 = pow_1_3(t169);
        let t173 = piecewise3(t170, t23, t171 * t169);
        let t174 = t173 * t27;
        let t175 = rho1 * rho1;
        let t176 = pow_1_3(rho1);
        let t177 = t176 * t176;
        let t179 = 1.0 / t177 / t175;
        let t180 = sigma2 * t179;
        let t183 = 0.804e0 + 0.91464571985215458336e-2 * t34 * t180;
        let t186 = 0.1804e1 - 0.646416e0 / t183;
        let t188 = 1.0 / t177 / rho1;
        let t189 = tau1 * t188;
        let t190 = t51 - t189;
        let t191 = t48 * t190;
        let t192 = t51 + t189;
        let t193 = 1.0 / t192;
        let t195 = t190 * t190;
        let t196 = t60 * t195;
        let t197 = t192 * t192;
        let t198 = 1.0 / t197;
        let t200 = t195 * t190;
        let t201 = t66 * t200;
        let t202 = t197 * t192;
        let t203 = 1.0 / t202;
        let t205 = t195 * t195;
        let t206 = t72 * t205;
        let t207 = t197 * t197;
        let t208 = 1.0 / t207;
        let t210 = t205 * t190;
        let t211 = t78 * t210;
        let t212 = t207 * t192;
        let t213 = 1.0 / t212;
        let t215 = t205 * t195;
        let t216 = t84 * t215;
        let t217 = t207 * t197;
        let t218 = 1.0 / t217;
        let t220 = t205 * t200;
        let t221 = t90 * t220;
        let t222 = t207 * t202;
        let t223 = 1.0 / t222;
        let t225 = t205 * t205;
        let t226 = t96 * t225;
        let t227 = t207 * t207;
        let t228 = 1.0 / t227;
        let t230 = t225 * t190;
        let t231 = t102 * t230;
        let t233 = 1.0 / t227 / t192;
        let t235 = t225 * t195;
        let t236 = t108 * t235;
        let t238 = 1.0 / t227 / t197;
        let t241 = t114 * t225 * t200;
        let t243 = 1.0 / t227 / t202;
        let t245 = t191 * t193 + t196 * t198 + t201 * t203 + t206 * t208 + t211 * t213 + t216 * t218 + t221 * t223 + t226 * t228 + t231 * t233 + t236 * t238 + t241 * t243 + t47;
        let t249 = 1.0 + 0.186726e-2 * t180 + 0.373452e-2 * t189 - t125;
        let t252 = t129 * sigma2;
        let t255 = 2.0 * t189 - t134;
        let t257 = t132 * t255 + t252 * t179;
        let t258 = t249 * t249;
        let t259 = 1.0 / t258;
        let t261 = sigma2 * sigma2;
        let t262 = t141 * t261;
        let t263 = t175 * t175;
        let t264 = t263 * rho1;
        let t266 = 1.0 / t176 / t264;
        let t268 = t149 * sigma2;
        let t271 = t255 * t255;
        let t273 = t268 * t179 * t255 + t153 * t271 + t262 * t266;
        let t274 = t258 * t249;
        let t275 = 1.0 / t274;
        let t277 = t186 * t245 + t122 / t249 + t257 * t259 + t273 * t275;
        let t281 = piecewise3(t165, 0.0, -3.0 / 8.0 * t6 * t174 * t277);
        let tzk0 = t164 + t281;
        zk[ip] += tzk0;
    }
}
