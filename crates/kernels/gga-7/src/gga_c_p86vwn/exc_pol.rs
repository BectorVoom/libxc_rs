//! GGA_C_P86VWN exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 121 shared lines across all orders.
//! Delta: 121 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_p86vwn_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mdelta: f64,
    param_mgamma: f64,
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
        // --- shared preamble (121 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = f64::sqrt(t11);
        let t15 = t12 + 0.186372e1 * t13 + 0.129352e2;
        let t16 = 1.0 / t15;
        let t20 = f64::ln(t4 * t10 * t16 / 4.0);
        let t21 = 0.310907e-1 * t20;
        let t22 = t13 + 0.372744e1;
        let t25 = f64::atan(0.61519908197590802322e1 / t22);
        let t26 = 0.38783294878113014393e-1 * t25;
        let t27 = t13 / 2.0;
        let t28 = t27 + 0.10498e0;
        let t29 = t28 * t28;
        let t31 = f64::ln(t29 * t16);
        let t32 = 0.96902277115443742139e-3 * t31;
        let t33 = M_PI * M_PI;
        let t34 = 1.0 / t33;
        let t36 = t12 + 0.565535e0 * t13 + 0.130045e2;
        let t37 = 1.0 / t36;
        let t41 = f64::ln(t4 * t10 * t37 / 4.0);
        let t42 = t13 + 0.113107e1;
        let t45 = f64::atan(0.71231089178181179908e1 / t42);
        let t47 = t27 + 0.47584e-2;
        let t48 = t47 * t47;
        let t50 = f64::ln(t48 * t37);
        let t53 = t34 * (t41 + 0.317708004743941464e0 * t45 + 0.41403379428206274608e-3 * t50);
        let t54 = rho0 - rho1;
        let t55 = 1.0 / t7;
        let t56 = t54 * t55;
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(zeta_threshold);
        let t60 = t59 * zeta_threshold;
        let t61 = pow_1_3(t57);
        let t62 = t61 * t57;
        let t63 = piecewise3(t58, t60, t62);
        let t64 = 1.0 - t56;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t67 = t66 * t64;
        let t68 = piecewise3(t65, t60, t67);
        let t69 = t63 + t68 - 2.0;
        let t70 = t53 * t69;
        let t71 = M_CBRT2;
        let t72 = t71 - 1.0;
        let t74 = 1.0 / t72 / 2.0;
        let t75 = t54 * t54;
        let t76 = t75 * t75;
        let t77 = t7 * t7;
        let t78 = t77 * t77;
        let t79 = 1.0 / t78;
        let t83 = 9.0 * t72;
        let t84 = t74 * (-t76 * t79 + 1.0) * t83;
        let t86 = t70 * t84 / 24.0;
        let t88 = t12 + 0.353021e1 * t13 + 0.180578e2;
        let t89 = 1.0 / t88;
        let t93 = f64::ln(t4 * t10 * t89 / 4.0);
        let t95 = t13 + 0.706042e1;
        let t98 = f64::atan(0.473092690956011283e1 / t95);
        let t100 = t27 + 0.325e0;
        let t101 = t100 * t100;
        let t103 = f64::ln(t101 * t89);
        let t105 = 0.1554535e-1 * t93 + 0.52491393169780936218e-1 * t98 + 0.22478670955426118383e-2 * t103 - t21 - t26 - t32;
        let t106 = t105 * t69;
        let t107 = t74 * t76;
        let t108 = t107 * t79;
        let t109 = t106 * t108;
        let t111 = sigma0 + 2.0 * sigma1 + sigma2;
        let t113 = 1.0 / t8 / t77;
        let t114 = t111 * t113;
        let t115 = param_aa + param_bb;
        let t116 = param_ftilde * t115;
        let t117 = param_malpha * t1;
        let t118 = t3 * t6;
        let t119 = t118 * t9;
        let t122 = t1 * t1;
        let t123 = param_mbeta * t122;
        let t124 = t3 * t3;
        let t125 = t124 * t5;
        let t126 = t8 * t8;
        let t127 = 1.0 / t126;
        let t128 = t125 * t127;
        let t131 = param_bb + t117 * t119 / 4.0 + t123 * t128 / 4.0;
        let t132 = param_mgamma * t1;
        let t135 = param_mdelta * t122;
        let t140 = 1.0 + t132 * t119 / 4.0 + t135 * t128 / 4.0 + 0.23873241463784300365e4 * param_mbeta * t55;
        let t141 = 1.0 / t140;
        let t143 = t131 * t141 + param_aa;
        let t144 = 1.0 / t143;
        let t145 = f64::sqrt(t111);
        let t146 = t144 * t145;
        let t147 = f64::powf(t7, 1.0 / 6.0);
        let t149 = 1.0 / t147 / t7;
        let t152 = f64::exp(-t116 * t146 * t149);
        let t153 = t114 * t152;
        let t154 = t59 * t59;
        let t155 = t154 * zeta_threshold;
        let t156 = t61 * t61;
        let t157 = t156 * t57;
        let t158 = piecewise3(t58, t155, t157);
        let t159 = t66 * t66;
        let t160 = t159 * t64;
        let t161 = piecewise3(t65, t155, t160);
        let t162 = t158 + t161;
        let t163 = f64::sqrt(t162);
        let t164 = 1.0 / t163;
        let t165 = t143 * t164;
        let t166 = M_SQRT2;
        let t167 = t165 * t166;
        let t168 = t153 * t167;
        let tzk0 = t21 + t26 + t32 - t86 + t109 + t168;
        zk[ip] += tzk0;
    }
}
