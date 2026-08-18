//! GGA_X_KT vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_kt.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_kt_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_gamma: f64,
    param_delta: f64,
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
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
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
        let t28 = t2 * t2;
        let t29 = param_gamma * t28;
        let t31 = pow_1_3(1.0 / M_PI);
        let t32 = 1.0 / t31;
        let t33 = M_CBRT4;
        let t34 = t32 * t33;
        let t35 = t29 * t34;
        let t36 = M_CBRT2;
        let t37 = t36 * t36;
        let t38 = t19 * t6;
        let t39 = pow_1_3(t38);
        let t40 = t39 * t38;
        let t41 = t37 * t40;
        let t42 = rho0 * rho0;
        let t43 = pow_1_3(rho0);
        let t44 = t43 * t43;
        let t46 = 1.0 / t44 / t42;
        let t47 = sigma0 * t46;
        let t49 = t41 / 4.0 + param_delta;
        let t50 = 1.0 / t49;
        let t51 = t47 * t50;
        let t55 = 1.0 - t35 * t41 * t51 / 18.0;
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = t64 * t6;
        let t71 = pow_1_3(t70);
        let t72 = t71 * t70;
        let t73 = t37 * t72;
        let t74 = rho1 * rho1;
        let t75 = pow_1_3(rho1);
        let t76 = t75 * t75;
        let t78 = 1.0 / t76 / t74;
        let t79 = sigma2 * t78;
        let t81 = t73 / 4.0 + param_delta;
        let t82 = 1.0 / t81;
        let t83 = t79 * t82;
        let t87 = 1.0 - t35 * t73 * t83 / 18.0;
        let t91 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t87);
        let tzk0 = t59 + t91;
        zk[ip] += tzk0;
        let t92 = t6 * t6;
        let t93 = 1.0 / t92;
        let t94 = t16 * t93;
        let t96 = piecewise5(t10, 0.0, t14, 0.0, t7 - t94);
        let t99 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t96);
        let t100 = t99 * t26;
        let t104 = t26 * t26;
        let t105 = 1.0 / t104;
        let t106 = t25 * t105;
        let t109 = t5 * t106 * t55 / 8.0;
        let t111 = t29 * t34 * t37;
        let t112 = t39 * sigma0;
        let t113 = t46 * t50;
        let t115 = t96 * t6 + t18 + 1.0;
        let t116 = t113 * t115;
        let t122 = 1.0 / t44 / t42 / rho0;
        let t123 = sigma0 * t122;
        let t129 = t29 * t34 * t36;
        let t130 = t39 * t39;
        let t131 = t130 * t38;
        let t132 = t131 * sigma0;
        let t133 = t49 * t49;
        let t134 = 1.0 / t133;
        let t135 = t46 * t134;
        let t136 = t135 * t115;
        let t140 = -2.0 / 27.0 * t111 * t112 * t116 + 4.0 / 27.0 * t35 * t41 * t123 * t50 + t129 * t132 * t136 / 27.0;
        let t145 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t100 * t55 - t109 - 3.0 / 8.0 * t5 * t27 * t140);
        let t146 = t61 * t93;
        let t148 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t146);
        let t151 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t148);
        let t152 = t151 * t26;
        let t156 = t68 * t105;
        let t159 = t5 * t156 * t87 / 8.0;
        let t160 = t71 * sigma2;
        let t161 = t78 * t82;
        let t163 = t148 * t6 + t63 + 1.0;
        let t164 = t161 * t163;
        let t168 = t71 * t71;
        let t169 = t168 * t70;
        let t170 = t169 * sigma2;
        let t171 = t81 * t81;
        let t172 = 1.0 / t171;
        let t173 = t78 * t172;
        let t174 = t173 * t163;
        let t178 = -2.0 / 27.0 * t111 * t160 * t164 + t129 * t170 * t174 / 27.0;
        let t183 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t152 * t87 - t159 - 3.0 / 8.0 * t5 * t69 * t178);
        let tvrho0 = t59 + t91 + t6 * (t145 + t183);
        vrho[ip * 2] += tvrho0;
        let t187 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t94);
        let t190 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t187);
        let t191 = t190 * t26;
        let t196 = t187 * t6 + t18 + 1.0;
        let t197 = t113 * t196;
        let t201 = t135 * t196;
        let t205 = -2.0 / 27.0 * t111 * t112 * t197 + t129 * t132 * t201 / 27.0;
        let t210 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t191 * t55 - t109 - 3.0 / 8.0 * t5 * t27 * t205);
        let t212 = piecewise5(t14, 0.0, t10, 0.0, t7 - t146);
        let t215 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t212);
        let t216 = t215 * t26;
        let t221 = t212 * t6 + t63 + 1.0;
        let t222 = t161 * t221;
        let t228 = 1.0 / t76 / t74 / rho1;
        let t229 = sigma2 * t228;
        let t234 = t173 * t221;
        let t238 = -2.0 / 27.0 * t111 * t160 * t222 + 4.0 / 27.0 * t35 * t73 * t229 * t82 + t129 * t170 * t234 / 27.0;
        let t243 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t216 * t87 - t159 - 3.0 / 8.0 * t5 * t69 * t238);
        let tvrho1 = t59 + t91 + t6 * (t210 + t243);
        vrho[ip * 2 + 1] += tvrho1;
        let t246 = t4 * t25;
        let t248 = t26 * param_gamma * t32;
        let t249 = t246 * t248;
        let t250 = t33 * t37;
        let t253 = t250 * t40 * t46 * t50;
        let t256 = piecewise3(t1, 0.0, t249 * t253 / 16.0);
        let tvsigma0 = t6 * t256;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t257 = t4 * t68;
        let t258 = t257 * t248;
        let t261 = t250 * t72 * t78 * t82;
        let t264 = piecewise3(t60, 0.0, t258 * t261 / 16.0);
        let tvsigma2 = t6 * t264;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
