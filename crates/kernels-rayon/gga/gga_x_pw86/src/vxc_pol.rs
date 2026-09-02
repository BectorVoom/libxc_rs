//! GGA_X_PW86 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pw86_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
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
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_aa * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t43 = t28 * t28;
        let t44 = param_bb * t43;
        let t46 = 1.0 / t31 / t30;
        let t47 = sigma0 * sigma0;
        let t48 = t46 * t47;
        let t49 = t35 * t35;
        let t50 = t49 * rho0;
        let t52 = 1.0 / t36 / t50;
        let t56 = t30 * t30;
        let t58 = param_cc / t56;
        let t59 = t47 * sigma0;
        let t60 = t49 * t49;
        let t61 = 1.0 / t60;
        let t65 = 1.0 + t29 * t34 * t39 / 24.0 + t44 * t48 * t52 / 576.0 + t58 * t59 * t61 / 2304.0;
        let t66 = rmath::pow(t65, 1.0 / 15.0);
        let t70 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t16;
        let t74 = piecewise5(t14, t11, t10, t15, t72 * t7);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t22, t77 * t75);
        let t80 = t79 * t26;
        let t81 = t33 * sigma2;
        let t82 = rho1 * rho1;
        let t83 = pow_1_3(rho1);
        let t84 = t83 * t83;
        let t86 = 1.0 / t84 / t82;
        let t90 = sigma2 * sigma2;
        let t91 = t46 * t90;
        let t92 = t82 * t82;
        let t93 = t92 * rho1;
        let t95 = 1.0 / t83 / t93;
        let t99 = t90 * sigma2;
        let t100 = t92 * t92;
        let t101 = 1.0 / t100;
        let t105 = 1.0 + t29 * t81 * t86 / 24.0 + t44 * t91 * t95 / 576.0 + t58 * t99 * t101 / 2304.0;
        let t106 = rmath::pow(t105, 1.0 / 15.0);
        let t110 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t80 * t106);
        let tzk0 = t70 + t110;
        zk[ip] += tzk0;
        let t111 = t6 * t6;
        let t112 = 1.0 / t111;
        let t113 = t16 * t112;
        let t115 = piecewise5(t10, 0.0, t14, 0.0, t7 - t113);
        let t118 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t115);
        let t123 = t26 * t26;
        let t124 = 1.0 / t123;
        let t128 = t5 * t25 * t124 * t66 / 8.0;
        let t129 = t5 * t25;
        let t130 = t66 * t66;
        let t131 = t130 * t130;
        let t133 = t131 * t131;
        let t134 = t133 * t131 * t130;
        let t135 = 1.0 / t134;
        let t136 = t26 * t135;
        let t137 = t35 * rho0;
        let t139 = 1.0 / t37 / t137;
        let t143 = t49 * t35;
        let t145 = 1.0 / t36 / t143;
        let t149 = t60 * rho0;
        let t150 = 1.0 / t149;
        let t154 = -t29 * t34 * t139 / 9.0 - t44 * t48 * t145 / 108.0 - t58 * t59 * t150 / 288.0;
        let t155 = t136 * t154;
        let t159 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t118 * t26 * t66 - t128 - t129 * t155 / 40.0);
        let t160 = t72 * t112;
        let t162 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t160);
        let t165 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t162);
        let t173 = t5 * t79 * t124 * t106 / 8.0;
        let t175 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t165 * t26 * t106 - t173);
        let tvrho0 = t70 + t110 + t6 * (t159 + t175);
        vrho[ip * 2] += tvrho0;
        let t179 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t113);
        let t182 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t179);
        let t188 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t182 * t26 * t66 - t128);
        let t190 = piecewise5(t14, 0.0, t10, 0.0, t7 - t160);
        let t193 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t190);
        let t198 = t5 * t79;
        let t199 = t106 * t106;
        let t200 = t199 * t199;
        let t202 = t200 * t200;
        let t203 = t202 * t200 * t199;
        let t204 = 1.0 / t203;
        let t205 = t26 * t204;
        let t206 = t82 * rho1;
        let t208 = 1.0 / t84 / t206;
        let t212 = t92 * t82;
        let t214 = 1.0 / t83 / t212;
        let t218 = t100 * rho1;
        let t219 = 1.0 / t218;
        let t223 = -t29 * t81 * t208 / 9.0 - t44 * t91 * t214 / 108.0 - t58 * t99 * t219 / 288.0;
        let t224 = t205 * t223;
        let t228 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t193 * t26 * t106 - t173 - t198 * t224 / 40.0);
        let tvrho1 = t70 + t110 + t6 * (t188 + t228);
        vrho[ip * 2 + 1] += tvrho1;
        let t234 = t46 * sigma0;
        let t241 = t29 * t33 * t39 / 24.0 + t44 * t234 * t52 / 288.0 + t58 * t47 * t61 / 768.0;
        let t242 = t136 * t241;
        let t245 = piecewise3(t1, 0.0, -t129 * t242 / 40.0);
        let tvsigma0 = t6 * t245;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t249 = t46 * sigma2;
        let t256 = t29 * t33 * t86 / 24.0 + t44 * t249 * t95 / 288.0 + t58 * t90 * t101 / 768.0;
        let t257 = t205 * t256;
        let t260 = piecewise3(t71, 0.0, -t198 * t257 / 40.0);
        let tvsigma2 = t6 * t260;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
