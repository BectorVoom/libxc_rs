//! MGGA_X_BR89 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};
use libxc_kernel_math::br89::xc_mgga_x_br89_get_x;

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_br89_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_at: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = pow_1_3(rho[ip]);
        let t16 = t14 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t19 = 1.0 / t18;
        let t20 = M_CBRT4;
        let t21 = t19 * t20;
        let t22 = t16 * t21;
        let t23 = M_CBRT2;
        let t24 = t23 * t23;
        let t25 = t15 * t15;
        let t27 = 1.0 / t25 / rho[ip];
        let t30 = param_gamma * tau[ip];
        let t33 = param_gamma * sigma[ip];
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t25 / t34;
        let t40 = f64::abs(lapl[ip] * t27 / 2.0 - 2.0 * t30 * t27 + t33 * t36 / 4.0);
        let t43 = t24 * t40 / 3.0 < 0.5e-12;
        let t44 = lapl[ip] * t24;
        let t47 = t24 * t27;
        let t50 = t24 * t36;
        let t53 = t44 * t27 / 6.0 - 2.0 / 3.0 * t30 * t47 + t33 * t50 / 12.0;
        let t54 = 0.0 < t53;
        let t55 = piecewise3(t54, 0.5e-12, -0.5e-12);
        let t56 = piecewise3(t43, t55, t53);
        let t57 = xc_mgga_x_br89_get_x(t56);
        let t59 = f64::exp(t57 / 3.0);
        let t60 = f64::exp(-t57);
        let t62 = 1.0 + t57 / 2.0;
        let t63 = t60 * t62;
        let t64 = 1.0 - t63;
        let t65 = t59 * t64;
        let t66 = 1.0 / t57;
        let t67 = M_CBRT6;
        let t68 = t67 * t67;
        let t69 = M_PI * M_PI;
        let t70 = pow_1_3(t69);
        let t71 = t70 * t70;
        let t73 = 3.0 / 10.0 * t68 * t71;
        let t74 = tau[ip] * t24;
        let t75 = t74 * t27;
        let t76 = t73 - t75;
        let t77 = t73 + t75;
        let t78 = 1.0 / t77;
        let t80 = t76 * t76;
        let t81 = t80 * t76;
        let t82 = t77 * t77;
        let t83 = t82 * t77;
        let t84 = 1.0 / t83;
        let t87 = t80 * t80;
        let t88 = t87 * t76;
        let t89 = t82 * t82;
        let t91 = 1.0 / t89 / t77;
        let t95 = 1.0 + param_at * (t76 * t78 - 2.0 * t81 * t84 + t88 * t91);
        let t96 = t66 * t95;
        let t97 = t65 * t96;
        let t100 = piecewise3(t3, 0.0, -t22 * t97 / 4.0);
        let tzk0 = 2.0 * t100;
        zk[ip] += tzk0;
        let t102 = t14 / t25;
        let t103 = t102 * t21;
        let t106 = t16 * t19;
        let t107 = M_CBRTPI;
        let t108 = t107 * t107;
        let t109 = t20 * t108;
        let t110 = piecewise3(t54, 0.0, 0.0);
        let t115 = t34 * rho[ip];
        let t117 = 1.0 / t25 / t115;
        let t118 = t24 * t117;
        let t122 = piecewise3(t43, t110, -5.0 / 18.0 * t44 * t36 + 10.0 / 9.0 * t30 * t50 - 2.0 / 9.0 * t33 * t118);
        let t123 = t56 * t56;
        let t124 = 1.0 / t123;
        let t125 = t122 * t124;
        let t126 = t109 * t125;
        let t127 = t106 * t126;
        let t129 = f64::exp(-2.0 / 3.0 * t57);
        let t130 = 1.0 / t129;
        let t131 = t57 * t57;
        let t133 = t131 - 2.0 * t57 + 3.0;
        let t134 = 1.0 / t133;
        let t135 = t130 * t134;
        let t136 = t57 - 2.0;
        let t137 = t136 * t136;
        let t138 = t135 * t137;
        let t139 = t138 * t97;
        let t142 = t108 * t122;
        let t143 = t124 * t130;
        let t144 = t142 * t143;
        let t145 = t134 * t137;
        let t146 = t145 * t63;
        let t148 = t142 * t124;
        let t149 = t137 * t60;
        let t150 = t135 * t149;
        let t153 = t144 * t146 - t148 * t150 / 2.0;
        let t154 = t59 * t153;
        let t155 = t154 * t96;
        let t158 = t20 * t59;
        let t159 = 1.0 / t131;
        let t160 = t64 * t159;
        let t161 = t158 * t160;
        let t162 = t106 * t161;
        let t163 = t95 * t108;
        let t165 = t143 * t145;
        let t166 = t163 * t122 * t165;
        let t169 = t66 * param_at;
        let t173 = 1.0 / t82;
        let t174 = t76 * t173;
        let t175 = t74 * t36;
        let t178 = t80 * t84;
        let t181 = 1.0 / t89;
        let t182 = t81 * t181;
        let t185 = t87 * t91;
        let t189 = 1.0 / t89 / t82;
        let t190 = t88 * t189;
        let t193 = 5.0 / 3.0 * t74 * t36 * t78 + 5.0 / 3.0 * t174 * t175 - 10.0 * t178 * t175 - 10.0 * t182 * t175 + 25.0 / 3.0 * t185 * t175 + 25.0 / 3.0 * t190 * t175;
        let t194 = t169 * t193;
        let t195 = t65 * t194;
        let t199 = piecewise3(t3, 0.0, -t103 * t97 / 12.0 - t127 * t139 / 12.0 - t22 * t155 / 4.0 + t162 * t166 / 4.0 - t22 * t195 / 4.0);
        let tvrho0 = 2.0 * rho[ip] * t199 + 2.0 * t100;
        vrho[ip] += tvrho0;
        let t202 = param_gamma * t24;
        let t203 = t202 * t36;
        let t205 = piecewise3(t43, t110, t203 / 12.0);
        let t206 = t205 * t124;
        let t207 = t109 * t206;
        let t208 = t106 * t207;
        let t211 = t108 * t205;
        let t212 = t211 * t143;
        let t214 = t211 * t124;
        let t217 = t212 * t146 - t214 * t150 / 2.0;
        let t218 = t59 * t217;
        let t219 = t218 * t96;
        let t223 = t163 * t205 * t165;
        let t227 = piecewise3(t3, 0.0, -t208 * t139 / 12.0 - t22 * t219 / 4.0 + t162 * t223 / 4.0);
        let tvsigma0 = 2.0 * rho[ip] * t227;
        vsigma[ip] += tvsigma0;
        let t230 = piecewise3(t43, t110, t47 / 6.0);
        let t231 = t230 * t124;
        let t232 = t109 * t231;
        let t233 = t106 * t232;
        let t236 = t108 * t230;
        let t237 = t236 * t143;
        let t239 = t236 * t124;
        let t242 = t237 * t146 - t239 * t150 / 2.0;
        let t243 = t59 * t242;
        let t244 = t243 * t96;
        let t248 = t163 * t230 * t165;
        let t252 = piecewise3(t3, 0.0, -t233 * t139 / 12.0 - t22 * t244 / 4.0 + t162 * t248 / 4.0);
        let tvlapl0 = 2.0 * rho[ip] * t252;
        vlapl[ip] += tvlapl0;
        let t256 = piecewise3(t43, t110, -2.0 / 3.0 * t202 * t27);
        let t257 = t256 * t124;
        let t258 = t109 * t257;
        let t259 = t106 * t258;
        let t262 = t108 * t256;
        let t263 = t262 * t143;
        let t265 = t262 * t124;
        let t268 = t263 * t146 - t265 * t150 / 2.0;
        let t269 = t59 * t268;
        let t270 = t269 * t96;
        let t274 = t163 * t256 * t165;
        let t287 = -t174 * t47 + 6.0 * t178 * t47 + 6.0 * t182 * t47 - 5.0 * t185 * t47 - 5.0 * t190 * t47 - t47 * t78;
        let t288 = t169 * t287;
        let t289 = t65 * t288;
        let t293 = piecewise3(t3, 0.0, -t259 * t139 / 12.0 - t22 * t270 / 4.0 + t162 * t274 / 4.0 - t22 * t289 / 4.0);
        let tvtau0 = 2.0 * rho[ip] * t293;
        vtau[ip] += tvtau0;
    }
}
