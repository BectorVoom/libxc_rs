//! GGA_C_PBE_VWN exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 126 shared lines across all orders.
//! Delta: 126 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_pbe_vwn_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_BB: f64,
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
        // --- shared preamble (126 lines) ---
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
        let t110 = t59 * t59;
        let t111 = t61 * t61;
        let t112 = piecewise3(t58, t110, t111);
        let t113 = t66 * t66;
        let t114 = piecewise3(t65, t110, t113);
        let t116 = t112 / 2.0 + t114 / 2.0;
        let t117 = t116 * t116;
        let t118 = t117 * t116;
        let t119 = param_gamma * t118;
        let t121 = sigma0 + 2.0 * sigma1 + sigma2;
        let t123 = 1.0 / t8 / t77;
        let t124 = t121 * t123;
        let t126 = 1.0 / t117;
        let t127 = t1 * t1;
        let t129 = 1.0 / t3;
        let t130 = t129 * t5;
        let t131 = t126 * t127 * t130;
        let t134 = param_BB * param_beta;
        let t135 = 1.0 / param_gamma;
        let t137 = (t21 + t26 + t32 - t86 + t109) * t135;
        let t138 = 1.0 / t118;
        let t140 = f64::exp(-t137 * t138);
        let t141 = t140 - 1.0;
        let t142 = 1.0 / t141;
        let t143 = t135 * t142;
        let t144 = t121 * t121;
        let t146 = t134 * t143 * t144;
        let t147 = t8 * t8;
        let t149 = 1.0 / t147 / t78;
        let t150 = t71 * t71;
        let t151 = t149 * t150;
        let t152 = t117 * t117;
        let t153 = 1.0 / t152;
        let t154 = t151 * t153;
        let t155 = t3 * t3;
        let t156 = 1.0 / t155;
        let t157 = t1 * t156;
        let t158 = t157 * t6;
        let t159 = t154 * t158;
        let t162 = t124 * t71 * t131 / 96.0 + t146 * t159 / 3072.0;
        let t163 = param_beta * t162;
        let t164 = param_beta * t135;
        let t167 = t164 * t142 * t162 + 1.0;
        let t168 = 1.0 / t167;
        let t169 = t135 * t168;
        let t171 = t163 * t169 + 1.0;
        let t172 = f64::ln(t171);
        let t173 = t119 * t172;
        let tzk0 = t21 + t26 + t32 - t86 + t109 + t173;
        zk[ip] += tzk0;
    }
}
