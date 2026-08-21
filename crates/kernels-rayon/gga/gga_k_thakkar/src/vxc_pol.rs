//! GGA_K_THAKKAR vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_thakkar.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_thakkar_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
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
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t37 = sigma0 * t36;
        let t38 = rmath::sqrt(sigma0);
        let t40 = 1.0 / t33 / rho0;
        let t41 = t38 * t40;
        let t42 = rmath::ln(t41 + rmath::sqrt(t41 * t41 + 1.0));
        let t45 = 1.0 + 0.0253 * t41 * t42;
        let t46 = 1.0 / t45;
        let t49 = M_CBRT4;
        let t50 = t49 * t38;
        let t53 = 2.0 * t50 * t40 + 1.0;
        let t54 = 1.0 / t53;
        let t57 = 1.0 + 0.0055 * t37 * t46 - 0.072 * t41 * t54;
        let t61 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t57);
        let t62 = rho1 <= dens_threshold;
        let t63 = -t17;
        let t65 = piecewise5(t15, t12, t11, t16, t63 * t8);
        let t66 = 1.0 + t65;
        let t67 = t66 <= zeta_threshold;
        let t68 = pow_1_3(t66);
        let t69 = t68 * t68;
        let t71 = piecewise3(t67, t24, t69 * t66);
        let t72 = t71 * t30;
        let t73 = rho1 * rho1;
        let t74 = pow_1_3(rho1);
        let t75 = t74 * t74;
        let t77 = 1.0 / t75 / t73;
        let t78 = sigma2 * t77;
        let t79 = rmath::sqrt(sigma2);
        let t81 = 1.0 / t74 / rho1;
        let t82 = t79 * t81;
        let t83 = rmath::ln(t82 + rmath::sqrt(t82 * t82 + 1.0));
        let t86 = 1.0 + 0.0253 * t82 * t83;
        let t87 = 1.0 / t86;
        let t90 = t49 * t79;
        let t93 = 2.0 * t90 * t81 + 1.0;
        let t94 = 1.0 / t93;
        let t97 = 1.0 + 0.0055 * t78 * t87 - 0.072 * t82 * t94;
        let t101 = piecewise3(t62, 0.0, 3.0 / 20.0 * t6 * t72 * t97);
        let tzk0 = t61 + t101;
        zk[ip] += tzk0;
        let t102 = t7 * t7;
        let t103 = 1.0 / t102;
        let t104 = t17 * t103;
        let t106 = piecewise5(t11, 0.0, t15, 0.0, t8 - t104);
        let t109 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t106);
        let t110 = t109 * t30;
        let t114 = 1.0 / t29;
        let t115 = t28 * t114;
        let t118 = t6 * t115 * t57 / 10.0;
        let t119 = t32 * rho0;
        let t121 = 1.0 / t34 / t119;
        let t122 = sigma0 * t121;
        let t125 = t45 * t45;
        let t126 = 1.0 / t125;
        let t128 = 1.0 / t33 / t32;
        let t129 = t38 * t128;
        let t132 = t37 + 1.0;
        let t133 = rmath::sqrt(t132);
        let t134 = 1.0 / t133;
        let t137 = -0.03373333333333333 * t129 * t42 - 0.03373333333333333 * t122 * t134;
        let t138 = t126 * t137;
        let t143 = t53 * t53;
        let t144 = 1.0 / t143;
        let t145 = t144 * t49;
        let t148 = -0.014666666666666666 * t122 * t46 - 0.0055 * t37 * t138 + 0.096 * t129 * t54 - 0.192 * t122 * t145;
        let t153 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t110 * t57 + t118 + 3.0 / 20.0 * t6 * t31 * t148);
        let t154 = t63 * t103;
        let t156 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t154);
        let t159 = piecewise3(t67, 0.0, 5.0 / 3.0 * t69 * t156);
        let t160 = t159 * t30;
        let t164 = t71 * t114;
        let t167 = t6 * t164 * t97 / 10.0;
        let t169 = piecewise3(t62, 0.0, 3.0 / 20.0 * t6 * t160 * t97 + t167);
        let tvrho0 = t61 + t101 + t7 * (t153 + t169);
        vrho[ip * 2] += tvrho0;
        let t173 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t104);
        let t176 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t173);
        let t177 = t176 * t30;
        let t182 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t177 * t57 + t118);
        let t184 = piecewise5(t15, 0.0, t11, 0.0, t8 - t154);
        let t187 = piecewise3(t67, 0.0, 5.0 / 3.0 * t69 * t184);
        let t188 = t187 * t30;
        let t192 = t73 * rho1;
        let t194 = 1.0 / t75 / t192;
        let t195 = sigma2 * t194;
        let t198 = t86 * t86;
        let t199 = 1.0 / t198;
        let t201 = 1.0 / t74 / t73;
        let t202 = t79 * t201;
        let t205 = t78 + 1.0;
        let t206 = rmath::sqrt(t205);
        let t207 = 1.0 / t206;
        let t210 = -0.03373333333333333 * t202 * t83 - 0.03373333333333333 * t195 * t207;
        let t211 = t199 * t210;
        let t216 = t93 * t93;
        let t217 = 1.0 / t216;
        let t218 = t217 * t49;
        let t221 = -0.014666666666666666 * t195 * t87 - 0.0055 * t78 * t211 + 0.096 * t202 * t94 - 0.192 * t195 * t218;
        let t226 = piecewise3(t62, 0.0, 3.0 / 20.0 * t6 * t188 * t97 + t167 + 3.0 / 20.0 * t6 * t72 * t221);
        let tvrho1 = t61 + t101 + t7 * (t182 + t226);
        vrho[ip * 2 + 1] += tvrho1;
        let t231 = 1.0 / t38;
        let t232 = t231 * t40;
        let t237 = 0.01265 * t232 * t42 + 0.01265 * t36 * t134;
        let t238 = t126 * t237;
        let t246 = 0.0055 * t36 * t46 - 0.0055 * t37 * t238 - 0.036 * t232 * t54 + 0.072 * t36 * t144 * t49;
        let t250 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t246);
        let tvsigma0 = t7 * t250;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t253 = 1.0 / t79;
        let t254 = t253 * t81;
        let t259 = 0.01265 * t254 * t83 + 0.01265 * t77 * t207;
        let t260 = t199 * t259;
        let t268 = 0.0055 * t77 * t87 - 0.0055 * t78 * t260 - 0.036 * t254 * t94 + 0.072 * t77 * t217 * t49;
        let t272 = piecewise3(t62, 0.0, 3.0 / 20.0 * t6 * t72 * t268);
        let tvsigma2 = t7 * t272;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
