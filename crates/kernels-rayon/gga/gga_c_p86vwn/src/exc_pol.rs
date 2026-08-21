//! GGA_C_P86VWN exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86vwn.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_p86vwn_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mgamma: f64,
    param_mdelta: f64,
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
        let t13 = rmath::sqrt(t11);
        let t15 = t12 + 1.86372 * t13 + 12.9352;
        let t16 = 1.0 / t15;
        let t20 = rmath::ln(t4 * t10 * t16 / 4.0);
        let t21 = 0.0310907 * t20;
        let t22 = t13 + 3.72744;
        let t25 = rmath::atan(6.15199081975908 / t22);
        let t26 = 0.038783294878113016 * t25;
        let t27 = t13 / 2.0;
        let t28 = t27 + 0.10498;
        let t29 = t28 * t28;
        let t31 = rmath::ln(t29 * t16);
        let t32 = 0.0009690227711544374 * t31;
        let t33 = M_PI * M_PI;
        let t34 = 1.0 / t33;
        let t36 = t12 + 0.565535 * t13 + 13.0045;
        let t37 = 1.0 / t36;
        let t41 = rmath::ln(t4 * t10 * t37 / 4.0);
        let t42 = t13 + 1.13107;
        let t45 = rmath::atan(7.123108917818118 / t42);
        let t47 = t27 + 0.0047584;
        let t48 = t47 * t47;
        let t50 = rmath::ln(t48 * t37);
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
        let t93 = rmath::ln(t4 * t10 * t89 / 4.0);
        let t95 = t13 + 7.06042;
        let t98 = rmath::atan(4.730926909560113 / t95);
        let t100 = t27 + 0.325;
        let t101 = t100 * t100;
        let t103 = rmath::ln(t101 * t89);
        let t105 = 0.01554535 * t93 + 0.05249139316978094 * t98 + 0.0022478670955426118 * t103 - t21 - t26 - t32;
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
        let t140 = 1.0 + t132 * t119 / 4.0 + t135 * t128 / 4.0 + 2387.32414637843 * param_mbeta * t55;
        let t141 = 1.0 / t140;
        let t143 = t131 * t141 + param_aa;
        let t144 = 1.0 / t143;
        let t145 = rmath::sqrt(t111);
        let t146 = t144 * t145;
        let t147 = rmath::pow(t7, 1.0 / 6.0);
        let t149 = 1.0 / t147 / t7;
        let t152 = rmath::exp(-t116 * t146 * t149);
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
        let t163 = rmath::sqrt(t162);
        let t164 = 1.0 / t163;
        let t165 = t143 * t164;
        let t166 = M_SQRT2;
        let t167 = t165 * t166;
        let t168 = t153 * t167;
        let tzk0 = t21 + t26 + t32 - t86 + t109 + t168;
        zk[ip] += tzk0;
    }
}
