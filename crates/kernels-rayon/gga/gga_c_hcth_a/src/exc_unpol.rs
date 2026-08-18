//! GGA_C_HCTH_A exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_hcth_a.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_hcth_a_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = piecewise3(t3, zeta_threshold, 1.0);
        let t6 = M_CBRT3;
        let t7 = 1.0 / M_PI;
        let t8 = pow_1_3(t7);
        let t9 = t6 * t8;
        let t10 = M_CBRT4;
        let t11 = t10 * t10;
        let t12 = t9 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t14 = 1.0 / t13;
        let t15 = M_CBRT2;
        let t16 = t14 * t15;
        let t17 = pow_1_3(zeta_threshold);
        let t19 = piecewise3(t3, 1.0 / t17, 1.0);
        let t21 = t12 * t16 * t19;
        let t22 = t21 / 4.0;
        let t23 = f64::sqrt(t21);
        let t25 = t22 + 1.86372 * t23 + 12.9352;
        let t26 = 1.0 / t25;
        let t27 = t19 * t26;
        let t31 = f64::ln(t12 * t16 * t27 / 4.0);
        let t32 = 0.0310907 * t31;
        let t33 = t23 + 3.72744;
        let t36 = f64::atan(6.15199081975908 / t33);
        let t37 = 0.038783294878113016 * t36;
        let t38 = t23 / 2.0;
        let t39 = t38 + 0.10498;
        let t40 = t39 * t39;
        let t42 = f64::ln(t40 * t26);
        let t43 = 0.0009690227711544374 * t42;
        let t45 = t22 + 3.53021 * t23 + 18.0578;
        let t46 = 1.0 / t45;
        let t47 = t19 * t46;
        let t51 = f64::ln(t12 * t16 * t47 / 4.0);
        let t53 = t23 + 7.06042;
        let t56 = f64::atan(4.730926909560113 / t53);
        let t58 = t38 + 0.325;
        let t59 = t58 * t58;
        let t61 = f64::ln(t59 * t46);
        let t65 = t17 * zeta_threshold;
        let t67 = piecewise3(2.0 <= zeta_threshold, t65, 2.0 * t15);
        let t69 = piecewise3(0.0 <= zeta_threshold, t65, 0.0);
        let t70 = t67 + t69 - 2.0;
        let t72 = t15 - 1.0;
        let t74 = 1.0 / t72 / 2.0;
        let t79 = piecewise3(t4, 0.0, t5 * (t32 + t37 + t43 + (0.01554535 * t51 + 0.05249139316978094 * t56 + 0.0022478670955426118 * t61 - t32 - t37 - t43) * t70 * t74) / 2.0);
        let t80 = t15 * t15;
        let t81 = sigma[ip] * t80;
        let t82 = rho[ip] * rho[ip];
        let t83 = t13 * t13;
        let t85 = 1.0 / t83 / t82;
        let t86 = t81 * t85;
        let t88 = 1.0 + 0.2 * t86;
        let t89 = 1.0 / t88;
        let t93 = sigma[ip] * sigma[ip];
        let t94 = t93 * t15;
        let t95 = t82 * t82;
        let t96 = t95 * rho[ip];
        let t98 = 1.0 / t13 / t96;
        let t99 = t88 * t88;
        let t100 = 1.0 / t99;
        let t101 = t98 * t100;
        let t104 = t93 * sigma[ip];
        let t105 = t95 * t95;
        let t106 = 1.0 / t105;
        let t107 = t104 * t106;
        let t108 = t99 * t88;
        let t109 = 1.0 / t108;
        let t112 = 0.0136823 + 0.053784 * t81 * t85 * t89 - 0.04406152 * t94 * t101 + 0.03326304 * t107 * t109;
        let t114 = 2.0 * t79 * t112;
        let t115 = t11 * t14;
        let t116 = t9 * t115;
        let t117 = t116 / 4.0;
        let t118 = f64::sqrt(t116);
        let t120 = t117 + 1.86372 * t118 + 12.9352;
        let t121 = 1.0 / t120;
        let t125 = f64::ln(t9 * t115 * t121 / 4.0);
        let t127 = t118 + 3.72744;
        let t130 = f64::atan(6.15199081975908 / t127);
        let t132 = t118 / 2.0;
        let t133 = t132 + 0.10498;
        let t134 = t133 * t133;
        let t136 = f64::ln(t134 * t121);
        let t138 = M_PI * M_PI;
        let t139 = 1.0 / t138;
        let t141 = t117 + 0.565535 * t118 + 13.0045;
        let t142 = 1.0 / t141;
        let t146 = f64::ln(t9 * t115 * t142 / 4.0);
        let t147 = t118 + 1.13107;
        let t150 = f64::atan(7.123108917818118 / t147);
        let t152 = t132 + 0.0047584;
        let t153 = t152 * t152;
        let t155 = f64::ln(t153 * t142);
        let t159 = piecewise3(t3, t65, 1.0);
        let t164 = 9.0 * (2.0 * t159 - 2.0) * t74 * t72;
        let t168 = 0.0310907 * t125 + 0.038783294878113016 * t130 + 0.0009690227711544374 * t136 - t139 * (t146 + 0.31770800474394145 * t150 + 0.00041403379428206277 * t155) * t164 / 24.0 - 2.0 * t79;
        let t170 = 1.0 + 0.006 * t86;
        let t171 = 1.0 / t170;
        let t175 = t170 * t170;
        let t176 = 1.0 / t175;
        let t177 = t98 * t176;
        let t180 = t175 * t170;
        let t181 = 1.0 / t180;
        let t184 = 0.836897 + 0.01032306 * t81 * t85 * t171 - 0.00020051856 * t94 * t177 - 3.95283456e-06 * t107 * t181;
        let t185 = t168 * t184;
        let tzk0 = t114 + t185;
        zk[ip] += tzk0;
    }
}
