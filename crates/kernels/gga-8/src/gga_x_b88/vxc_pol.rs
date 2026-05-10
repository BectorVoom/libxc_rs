//! GGA_X_B88 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 62 shared lines across all orders.
//! Delta: 71 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_b88_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_beta: f64,
    param_gamma: f64,
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
        // --- shared preamble (62 lines) ---
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
        let t28 = t2 * t2;
        let t29 = param_beta * t28;
        let t31 = pow_1_3(1.0 / M_PI);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = M_CBRT4;
        let t35 = t34 * sigma0;
        let t36 = rho0 * rho0;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / t36;
        let t41 = param_gamma * param_beta;
        let t42 = f64::sqrt(sigma0);
        let t44 = 1.0 / t37 / rho0;
        let t45 = t42 * t44;
        let t46 = f64::ln(t45 + f64::sqrt(t45 * t45 + 1.0));
        let t49 = t41 * t45 * t46 + 1.0;
        let t50 = 1.0 / t49;
        let t55 = 1.0 + 2.0 / 9.0 * t33 * t35 * t40 * t50;
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = t34 * sigma2;
        let t71 = rho1 * rho1;
        let t72 = pow_1_3(rho1);
        let t73 = t72 * t72;
        let t75 = 1.0 / t73 / t71;
        let t76 = f64::sqrt(sigma2);
        let t78 = 1.0 / t72 / rho1;
        let t79 = t76 * t78;
        let t80 = f64::ln(t79 + f64::sqrt(t79 * t79 + 1.0));
        let t83 = t41 * t79 * t80 + 1.0;
        let t84 = 1.0 / t83;
        let t89 = 1.0 + 2.0 / 9.0 * t33 * t70 * t75 * t84;
        let t93 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t89);
        let tzk0 = t59 + t93;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (71 lines) ---
        let t94 = t6 * t6;
        let t95 = 1.0 / t94;
        let t96 = t16 * t95;
        let t98 = piecewise5(t10, 0.0, t14, 0.0, t7 - t96);
        let t101 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t98);
        let t102 = t101 * t26;
        let t106 = t26 * t26;
        let t107 = 1.0 / t106;
        let t108 = t25 * t107;
        let t111 = t5 * t108 * t55 / 8.0;
        let t112 = t36 * rho0;
        let t114 = 1.0 / t38 / t112;
        let t119 = t32 * t34;
        let t120 = t29 * t119;
        let t121 = sigma0 * t40;
        let t122 = t49 * t49;
        let t123 = 1.0 / t122;
        let t125 = 1.0 / t37 / t36;
        let t129 = sigma0 * t114;
        let t130 = t121 + 1.0;
        let t131 = f64::sqrt(t130);
        let t132 = 1.0 / t131;
        let t136 = -4.0 / 3.0 * t41 * t42 * t125 * t46 - 4.0 / 3.0 * t41 * t129 * t132;
        let t137 = t123 * t136;
        let t141 = -16.0 / 27.0 * t33 * t35 * t114 * t50 - 2.0 / 9.0 * t120 * t121 * t137;
        let t146 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t102 * t55 - t111 - 3.0 / 8.0 * t5 * t27 * t141);
        let t147 = t61 * t95;
        let t149 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t147);
        let t152 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t149);
        let t153 = t152 * t26;
        let t157 = t68 * t107;
        let t160 = t5 * t157 * t89 / 8.0;
        let t162 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t153 * t89 - t160);
        let tvrho0 = t59 + t93 + t6 * (t146 + t162);
        vrho[ip * 2] += tvrho0;
        let t166 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t96);
        let t169 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t166);
        let t170 = t169 * t26;
        let t175 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t170 * t55 - t111);
        let t177 = piecewise5(t14, 0.0, t10, 0.0, t7 - t147);
        let t180 = piecewise3(t65, 0.0, 4.0 / 3.0 * t66 * t177);
        let t181 = t180 * t26;
        let t185 = t71 * rho1;
        let t187 = 1.0 / t73 / t185;
        let t192 = sigma2 * t75;
        let t193 = t83 * t83;
        let t194 = 1.0 / t193;
        let t196 = 1.0 / t72 / t71;
        let t200 = sigma2 * t187;
        let t201 = t192 + 1.0;
        let t202 = f64::sqrt(t201);
        let t203 = 1.0 / t202;
        let t207 = -4.0 / 3.0 * t41 * t76 * t196 * t80 - 4.0 / 3.0 * t41 * t200 * t203;
        let t208 = t194 * t207;
        let t212 = -16.0 / 27.0 * t33 * t70 * t187 * t84 - 2.0 / 9.0 * t120 * t192 * t208;
        let t217 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t181 * t89 - t160 - 3.0 / 8.0 * t5 * t69 * t212);
        let tvrho1 = t59 + t93 + t6 * (t175 + t217);
        vrho[ip * 2 + 1] += tvrho1;
        let t220 = t34 * t40;
        let t223 = 1.0 / t42;
        let t230 = t41 * t223 * t44 * t46 / 2.0 + t41 * t40 * t132 / 2.0;
        let t231 = t123 * t230;
        let t235 = -2.0 / 9.0 * t120 * t121 * t231 + 2.0 / 9.0 * t33 * t220 * t50;
        let t239 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t235);
        let tvsigma0 = t6 * t239;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t240 = t34 * t75;
        let t243 = 1.0 / t76;
        let t250 = t41 * t243 * t78 * t80 / 2.0 + t41 * t75 * t203 / 2.0;
        let t251 = t194 * t250;
        let t255 = -2.0 / 9.0 * t120 * t192 * t251 + 2.0 / 9.0 * t33 * t240 * t84;
        let t259 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t255);
        let tvsigma2 = t6 * t259;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
