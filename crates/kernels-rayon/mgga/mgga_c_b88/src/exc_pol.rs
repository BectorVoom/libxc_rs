//! MGGA_C_B88 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_b88_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 - rho1;
        let t3 = t2 * t2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = 1.0 / t5;
        let t8 = -t3 * t6 + 1.0;
        let t9 = t8 * t4;
        let t10 = rho0 <= dens_threshold;
        let t11 = M_CBRT3;
        let t12 = t11 * t11;
        let t13 = 1.0 / M_PI;
        let t14 = pow_1_3(t13);
        let t15 = 1.0 / t14;
        let t16 = t12 * t15;
        let t17 = M_CBRT4;
        let t18 = t16 * t17;
        let t19 = M_CBRT2;
        let t20 = 1.0 / t4;
        let t23 = 2.0 * rho0 * t20 <= zeta_threshold;
        let t24 = zeta_threshold - 1.0;
        let t27 = 2.0 * rho1 * t20 <= zeta_threshold;
        let t28 = -t24;
        let t29 = t2 * t20;
        let t30 = piecewise5(t23, t24, t27, t28, t29);
        let t31 = 1.0 + t30;
        let t32 = t31 * t4;
        let t33 = pow_1_3(t32);
        let t34 = 1.0 / t33;
        let t35 = t19 * t34;
        let t36 = rho0 * rho0;
        let t37 = pow_1_3(rho0);
        let t38 = t37 * t37;
        let t40 = 1.0 / t38 / t36;
        let t41 = sigma0 * t40;
        let t43 = 1.0 + 0.7e-2 * t41;
        let t44 = f64::powf(t43, 1.0 / 5.0);
        let t45 = t44 * t44;
        let t46 = t45 * t45;
        let t47 = 1.0 / t46;
        let t51 = 1.0 + 0.83333333333333333333e-3 * t18 * t41 * t47;
        let t52 = 1.0 / t51;
        let t54 = t18 * t35 * t52;
        let t56 = piecewise3(t10, 0.0, t54 / 9.0);
        let t57 = 0.63e0 * t56;
        let t58 = rho1 <= dens_threshold;
        let t59 = -t2;
        let t61 = piecewise5(t27, t24, t23, t28, t59 * t20);
        let t62 = 1.0 + t61;
        let t63 = t62 * t4;
        let t64 = pow_1_3(t63);
        let t65 = 1.0 / t64;
        let t66 = t19 * t65;
        let t67 = rho1 * rho1;
        let t68 = pow_1_3(rho1);
        let t69 = t68 * t68;
        let t71 = 1.0 / t69 / t67;
        let t72 = sigma2 * t71;
        let t74 = 1.0 + 0.7e-2 * t72;
        let t75 = f64::powf(t74, 1.0 / 5.0);
        let t76 = t75 * t75;
        let t77 = t76 * t76;
        let t78 = 1.0 / t77;
        let t82 = 1.0 + 0.83333333333333333333e-3 * t18 * t72 * t78;
        let t83 = 1.0 / t82;
        let t85 = t18 * t66 * t83;
        let t87 = piecewise3(t58, 0.0, t85 / 9.0);
        let t88 = 0.63e0 * t87;
        let t89 = t57 + t88;
        let t90 = 1.0 + t57 + t88;
        let t91 = f64::ln(t90);
        let t92 = t57 + t88 - t91;
        let t93 = t89 * t92;
        let t95 = 0.2e0 * t9 * t93;
        let t97 = 1.0 + t29 <= zeta_threshold;
        let t99 = 1.0 - t29 <= zeta_threshold;
        let t100 = piecewise5(t97, t24, t99, t28, t29);
        let t101 = 1.0 + t100;
        let t102 = t101 * t101;
        let t103 = pow_1_3(t101);
        let t104 = t103 * t103;
        let t106 = t19 * t19;
        let t107 = t104 * t102 * t106;
        let t108 = pow_1_3(t4);
        let t109 = t108 * t108;
        let t110 = t109 * t4;
        let t112 = 1.0 / t38 / rho0;
        let t116 = 2.0 * tau0 * t112 - t41 / 4.0;
        let t118 = t110 * t116 * t12;
        let t119 = t107 * t118;
        let t121 = 1.0 / t14 / t13;
        let t122 = t121 * t17;
        let t124 = 1.0 / t33 / t32;
        let t125 = t51 * t51;
        let t126 = t125 * t125;
        let t127 = 1.0 / t126;
        let t128 = t124 * t127;
        let t130 = 1.0 + 0.10666666666666666667e0 * t54;
        let t131 = f64::ln(t130);
        let t132 = t131 * t11;
        let t133 = t132 * t14;
        let t134 = t17 * t17;
        let t135 = t134 * t106;
        let t136 = t33 * t51;
        let t137 = t135 * t136;
        let t140 = 1.0 - 0.390625e0 * t133 * t137;
        let t142 = t122 * t128 * t140;
        let t145 = piecewise3(t10, 0.0, -0.18641351111111111112e-3 * t119 * t142);
        let t146 = piecewise5(t99, t24, t97, t28, -t29);
        let t147 = 1.0 + t146;
        let t148 = t147 * t147;
        let t149 = pow_1_3(t147);
        let t150 = t149 * t149;
        let t152 = t150 * t148 * t106;
        let t154 = 1.0 / t69 / rho1;
        let t158 = 2.0 * tau1 * t154 - t72 / 4.0;
        let t160 = t110 * t158 * t12;
        let t161 = t152 * t160;
        let t163 = 1.0 / t64 / t63;
        let t164 = t82 * t82;
        let t165 = t164 * t164;
        let t166 = 1.0 / t165;
        let t167 = t163 * t166;
        let t169 = 1.0 + 0.10666666666666666667e0 * t85;
        let t170 = f64::ln(t169);
        let t171 = t170 * t11;
        let t172 = t171 * t14;
        let t173 = t64 * t82;
        let t174 = t135 * t173;
        let t177 = 1.0 - 0.390625e0 * t172 * t174;
        let t179 = t122 * t167 * t177;
        let t182 = piecewise3(t58, 0.0, -0.18641351111111111112e-3 * t161 * t179);
        let tzk0 = -t95 + t145 + t182;
        zk[ip] += tzk0;
    }
}
