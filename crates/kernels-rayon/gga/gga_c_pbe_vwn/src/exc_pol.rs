//! GGA_C_PBE_VWN exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_vwn.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_pbe_vwn_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
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
        let t15 = t12 + 1.86372 * t13 + 12.9352;
        let t16 = 1.0 / t15;
        let t20 = f64::ln(t4 * t10 * t16 / 4.0);
        let t21 = 0.0310907 * t20;
        let t22 = t13 + 3.72744;
        let t25 = f64::atan(6.15199081975908 / t22);
        let t26 = 0.038783294878113016 * t25;
        let t27 = t13 / 2.0;
        let t28 = t27 + 0.10498;
        let t29 = t28 * t28;
        let t31 = f64::ln(t29 * t16);
        let t32 = 0.0009690227711544374 * t31;
        let t33 = M_PI * M_PI;
        let t34 = 1.0 / t33;
        let t36 = t12 + 0.565535 * t13 + 13.0045;
        let t37 = 1.0 / t36;
        let t41 = f64::ln(t4 * t10 * t37 / 4.0);
        let t42 = t13 + 1.13107;
        let t45 = f64::atan(7.123108917818118 / t42);
        let t47 = t27 + 0.0047584;
        let t48 = t47 * t47;
        let t50 = f64::ln(t48 * t37);
        let t53 = t34 * (t41 + 0.31770800474394145 * t45 + 0.00041403379428206277 * t50);
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
        let t88 = t12 + 3.53021 * t13 + 18.0578;
        let t89 = 1.0 / t88;
        let t93 = f64::ln(t4 * t10 * t89 / 4.0);
        let t95 = t13 + 7.06042;
        let t98 = f64::atan(4.730926909560113 / t95);
        let t100 = t27 + 0.325;
        let t101 = t100 * t100;
        let t103 = f64::ln(t101 * t89);
        let t105 = 0.01554535 * t93 + 0.05249139316978094 * t98 + 0.0022478670955426118 * t103 - t21 - t26 - t32;
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
