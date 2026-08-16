//! GGA_C_ACGGAP exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acggap.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_acggap_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = rho0 - rho1;
        let t35 = t34 * t34;
        let t36 = t35 * t35;
        let t37 = t7 * t7;
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t36 * t39;
        let t41 = 1.0 / t7;
        let t42 = t34 * t41;
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(zeta_threshold);
        let t46 = t45 * zeta_threshold;
        let t47 = pow_1_3(t43);
        let t48 = t47 * t43;
        let t49 = piecewise3(t44, t46, t48);
        let t50 = 1.0 - t42;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t53 = t52 * t50;
        let t54 = piecewise3(t51, t46, t53);
        let t55 = t49 + t54 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t55 * t59;
        let t62 = 1.0 + 0.5137e-1 * t11;
        let t67 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
        let t70 = 1.0 + 0.32163958997385070134e2 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.278125e-1 * t11;
        let t80 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t83 = 1.0 + 0.29608749977793437516e2 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.310907e-1 * t62 * t71 + t33 - 0.19751673498613801407e-1 * t85;
        let t88 = t60 * t87;
        let t89 = t40 * t88;
        let t91 = 0.19751673498613801407e-1 * t60 * t85;
        let t92 = f64::ln(2.0);
        let t93 = 1.0 - t92;
        let t94 = M_PI * M_PI;
        let t95 = 1.0 / t94;
        let t96 = t93 * t95;
        let t97 = t45 * t45;
        let t98 = t47 * t47;
        let t99 = piecewise3(t44, t97, t98);
        let t100 = t52 * t52;
        let t101 = piecewise3(t51, t97, t100);
        let t103 = t99 / 2.0 + t101 / 2.0;
        let t104 = t103 * t103;
        let t105 = t104 * t103;
        let t107 = 1.0 + 0.416675e-1 * t11;
        let t111 = 1.0 + 0.125e0 * t4 * t10 * t107;
        let t113 = 1.0 + 0.740825e-1 * t11;
        let t117 = 1.0 + 0.125e0 * t4 * t10 * t113;
        let t118 = 1.0 / t117;
        let t119 = t111 * t118;
        let t121 = sigma0 + 2.0 * sigma1 + sigma2;
        let t123 = 1.0 / t8 / t37;
        let t124 = t121 * t123;
        let t125 = 1.0 / t104;
        let t126 = t56 * t125;
        let t127 = t124 * t126;
        let t128 = 1.0 / t3;
        let t129 = t19 * t128;
        let t130 = f64::sqrt(t121);
        let t132 = 1.0 / t8 / t7;
        let t133 = t130 * t132;
        let t134 = t56 * t56;
        let t135 = 1.0 / t103;
        let t136 = t134 * t135;
        let t137 = 1.0 / t14;
        let t138 = t136 * t137;
        let t139 = t133 * t138;
        let t141 = 0.45e1 + t139 / 4.0;
        let t142 = t5 * t141;
        let t144 = 0.45e1 + 0.36675e0 * t139;
        let t145 = 1.0 / t144;
        let t146 = t142 * t145;
        let t147 = t129 * t146;
        let t150 = 1.0 / t93;
        let t151 = t119 * t150;
        let t153 = (-t33 + t89 + t91) * t150;
        let t154 = 1.0 / t105;
        let t155 = t94 * t154;
        let t157 = f64::exp(-t153 * t155);
        let t158 = t157 - 1.0;
        let t159 = 1.0 / t158;
        let t160 = t121 * t121;
        let t161 = t159 * t160;
        let t163 = 1.0 / t22 / t38;
        let t164 = t161 * t163;
        let t165 = t151 * t164;
        let t166 = t104 * t104;
        let t167 = 1.0 / t166;
        let t168 = t134 * t167;
        let t169 = t168 * t1;
        let t170 = 1.0 / t20;
        let t171 = t170 * t6;
        let t172 = t141 * t141;
        let t173 = t144 * t144;
        let t174 = 1.0 / t173;
        let t175 = t172 * t174;
        let t176 = t171 * t175;
        let t177 = t169 * t176;
        let t180 = t127 * t147 / 96.0 + 0.21437009059034868486e-3 * t165 * t177;
        let t181 = t180 * t150;
        let t182 = t150 * t159;
        let t183 = t182 * t180;
        let t186 = 1.0 + 0.65854491829355115987e0 * t119 * t183;
        let t187 = 1.0 / t186;
        let t188 = t181 * t187;
        let t191 = 1.0 + 0.65854491829355115987e0 * t119 * t188;
        let t192 = f64::ln(t191);
        let t194 = t96 * t105 * t192;
        let tzk0 = -t33 + t89 + t91 + t194;
        zk[ip] += tzk0;
    }
}
