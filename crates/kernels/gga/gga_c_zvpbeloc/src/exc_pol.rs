//! GGA_C_ZVPBELOC exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeloc.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_zvpbeloc_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t1 = f64::powf(4.0, 1.0 / 6.0);
        let t2 = t1 * t1;
        let t3 = t2 * t2;
        let t5 = f64::powf(3.0, 1.0 / 6.0);
        let t7 = M_PI * M_PI;
        let t8 = 1.0 / t7;
        let t9 = f64::powf(t8, 1.0 / 6.0);
        let t10 = t3 * t1 * t5 * t9;
        let t11 = 1.0 / M_PI;
        let t12 = pow_1_3::<f64>(t11);
        let t13 = rho0 + rho1;
        let t14 = pow_1_3::<f64>(t13);
        let t15 = 1.0 / t14;
        let t16 = t12 * t15;
        let t17 = rho0 - rho1;
        let t18 = t17 * t17;
        let t19 = t13 * t13;
        let t20 = 1.0 / t19;
        let t21 = t18 * t20;
        let t22 = 0.1e-19 < t21;
        let t23 = piecewise3::<f64>(t22, t21, 0.1e-19);
        let t27 = f64::exp(-0.99999999999999999999e0 * t10 * t16 * t23);
        let t28 = M_CBRT3;
        let t29 = t28 * t12;
        let t30 = M_CBRT4;
        let t31 = t30 * t30;
        let t33 = t29 * t31 * t15;
        let t35 = 1.0 + 0.53425e-1 * t33;
        let t36 = f64::sqrt(t33);
        let t39 = pow_3_2::<f64>(t33);
        let t41 = t28 * t28;
        let t42 = t12 * t12;
        let t43 = t41 * t42;
        let t44 = t14 * t14;
        let t47 = t43 * t30 / t44;
        let t49 = 0.379785e1 * t36 + 0.8969e0 * t33 + 0.204775e0 * t39 + 0.123235e0 * t47;
        let t52 = 1.0 + 0.16081979498692535067e2 / t49;
        let t53 = f64::ln(t52);
        let t55 = 0.621814e-1 * t35 * t53;
        let t56 = t18 * t18;
        let t57 = t19 * t19;
        let t58 = 1.0 / t57;
        let t59 = t56 * t58;
        let t60 = 1.0 / t13;
        let t61 = t17 * t60;
        let t62 = 1.0 + t61;
        let t63 = t62 <= zeta_threshold;
        let t64 = pow_1_3::<f64>(zeta_threshold);
        let t65 = t64 * zeta_threshold;
        let t66 = pow_1_3::<f64>(t62);
        let t67 = t66 * t62;
        let t68 = piecewise3::<f64>(t63, t65, t67);
        let t69 = 1.0 - t61;
        let t70 = t69 <= zeta_threshold;
        let t71 = pow_1_3::<f64>(t69);
        let t72 = t71 * t69;
        let t73 = piecewise3::<f64>(t70, t65, t72);
        let t74 = t68 + t73 - 2.0;
        let t75 = M_CBRT2;
        let t78 = 1.0 / (2.0 * t75 - 2.0);
        let t79 = t74 * t78;
        let t81 = 1.0 + 0.5137e-1 * t33;
        let t86 = 0.705945e1 * t36 + 0.1549425e1 * t33 + 0.420775e0 * t39 + 0.1562925e0 * t47;
        let t89 = 1.0 + 0.32163958997385070134e2 / t86;
        let t90 = f64::ln(t89);
        let t94 = 1.0 + 0.278125e-1 * t33;
        let t99 = 0.51785e1 * t36 + 0.905775e0 * t33 + 0.1100325e0 * t39 + 0.1241775e0 * t47;
        let t102 = 1.0 + 0.29608749977793437516e2 / t99;
        let t103 = f64::ln(t102);
        let t104 = t94 * t103;
        let t106 = -0.310907e-1 * t81 * t90 + t55 - 0.19751673498613801407e-1 * t104;
        let t107 = t79 * t106;
        let t108 = t59 * t107;
        let t110 = 0.19751673498613801407e-1 * t79 * t104;
        let t111 = f64::ln(2.0);
        let t112 = 1.0 - t111;
        let t113 = t112 * t8;
        let t114 = t64 * t64;
        let t115 = t66 * t66;
        let t116 = piecewise3::<f64>(t63, t114, t115);
        let t117 = t71 * t71;
        let t118 = piecewise3::<f64>(t70, t114, t117);
        let t120 = t116 / 2.0 + t118 / 2.0;
        let t121 = t120 * t120;
        let t122 = t121 * t120;
        let t124 = sigma0 + 2.0 * sigma1 + sigma2;
        let t126 = 1.0 / t14 / t19;
        let t127 = t124 * t126;
        let t128 = 1.0 / t121;
        let t129 = t75 * t128;
        let t131 = 1.0 / t12;
        let t132 = t41 * t131;
        let t134 = f64::exp(-t47 / 4.0);
        let t135 = 1.0 - t134;
        let t136 = t30 * t135;
        let t137 = t132 * t136;
        let t140 = 0.375e-1 + 0.83333333333333333332e-3 * t127 * t129 * t137;
        let t142 = t128 * t41;
        let t143 = t131 * t30;
        let t144 = t142 * t143;
        let t147 = 1.0 / t112;
        let t148 = t140 * t147;
        let t150 = (-t55 + t108 + t110) * t147;
        let t151 = 1.0 / t122;
        let t152 = t7 * t151;
        let t154 = f64::exp(-t150 * t152);
        let t155 = t154 - 1.0;
        let t156 = 1.0 / t155;
        let t157 = t7 * t156;
        let t158 = t124 * t124;
        let t159 = t157 * t158;
        let t160 = t148 * t159;
        let t162 = 1.0 / t44 / t57;
        let t163 = t75 * t75;
        let t164 = t162 * t163;
        let t165 = t121 * t121;
        let t166 = 1.0 / t165;
        let t168 = 1.0 / t42;
        let t169 = t28 * t168;
        let t170 = t169 * t31;
        let t171 = t164 * t166 * t170;
        let t174 = t127 * t75 * t144 / 96.0 + t160 * t171 / 3072.0;
        let t175 = t140 * t174;
        let t176 = t147 * t7;
        let t177 = t157 * t174;
        let t179 = t148 * t177 + 1.0;
        let t180 = 1.0 / t179;
        let t181 = t176 * t180;
        let t183 = t175 * t181 + 1.0;
        let t184 = f64::ln(t183);
        let t187 = t113 * t122 * t184 + t108 + t110 - t55;
        let tzk0 = t27 * t187;
        zk[ip] += tzk0;
    }
}
