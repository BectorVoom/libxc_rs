//! GGA_C_REVTCA exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_revtca.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_revtca_exc_pol(
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
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 1.0 + t4;
        let t6 = t5 <= zeta_threshold;
        let t7 = pow_1_3(zeta_threshold);
        let t8 = t7 * t7;
        let t9 = pow_1_3(t5);
        let t10 = t9 * t9;
        let t11 = piecewise3(t6, t8, t10);
        let t12 = 1.0 - t4;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3(t12);
        let t15 = t14 * t14;
        let t16 = piecewise3(t13, t8, t15);
        let t18 = t11 / 2.0 + t16 / 2.0;
        let t19 = t18 * t18;
        let t20 = t19 * t18;
        let t21 = M_CBRT3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t21 * t23;
        let t25 = M_CBRT4;
        let t26 = t25 * t25;
        let t27 = pow_1_3(t2);
        let t32 = 4.88827 + 0.79425925 * t24 * t26 / t27;
        let t33 = rmath::atan(t32);
        let t35 = -0.655868 * t33 + 0.897889;
        let t36 = t20 * t35;
        let t37 = t21 * t21;
        let t38 = 1.0 / t23;
        let t39 = t37 * t38;
        let t40 = t36 * t39;
        let t41 = t25 * t27;
        let t42 = M_CBRT6;
        let t43 = t42 * t42;
        let t44 = M_PI * M_PI;
        let t45 = pow_1_3(t44);
        let t46 = 1.0 / t45;
        let t47 = t43 * t46;
        let t48 = M_CBRT2;
        let t50 = sigma0 + 2.0 * sigma1 + sigma2;
        let t51 = rmath::sqrt(t50);
        let t52 = t48 * t51;
        let t53 = t27 * t2;
        let t54 = 1.0 / t53;
        let t56 = t47 * t52 * t54;
        let t57 = rmath::pow(t56, 2.3);
        let t59 = 1.0 + 0.004712150703442276 * t57;
        let t60 = 1.0 / t59;
        let t61 = t1 * t1;
        let t62 = t61 * t61;
        let t63 = t2 * t2;
        let t64 = t63 * t63;
        let t65 = 1.0 / t64;
        let t66 = t62 * t65;
        let t67 = M_CBRTPI;
        let t69 = pow_1_3(9.0);
        let t71 = t67 * M_PI * t69 * t47;
        let t73 = t3 * t37 * t38;
        let t76 = t71 * t52 * t73 / 36.0;
        let t77 = pow_1_4(f64::EPSILON);
        let t78 = t76 <= t77;
        let t79 = t67 * t67;
        let t81 = t69 * t69;
        let t83 = t45 * t45;
        let t84 = 1.0 / t83;
        let t85 = t42 * t84;
        let t86 = t79 * t44 * t81 * t85;
        let t87 = t48 * t48;
        let t88 = t87 * t50;
        let t89 = 1.0 / t63;
        let t91 = t23 * t23;
        let t92 = 1.0 / t91;
        let t97 = t44 * t44;
        let t104 = t67 * t97 * M_PI * t69 * t43 / t45 / t44;
        let t105 = t50 * t50;
        let t106 = t48 * t105;
        let t107 = t65 * t37;
        let t109 = 1.0 / t23 / t22;
        let t110 = t107 * t109;
        let t114 = t97 * t44;
        let t115 = t105 * t50;
        let t116 = t114 * t115;
        let t117 = t64 * t63;
        let t118 = 1.0 / t117;
        let t122 = t77 < t76;
        let t123 = piecewise3(t122, t76, t77);
        let t124 = rmath::sin(t123);
        let t125 = 1.0 / t123;
        let t126 = t124 * t125;
        let t127 = piecewise3(t78, 1.0 - t86 * t88 * t89 * t21 * t92 / 432.0 + t104 * t106 * t110 / 34560.0 - t116 * t118 / 322560.0, t126);
        let t128 = t127 * t127;
        let t129 = 1.0 - t128;
        let t131 = -t66 * t129 + 1.0;
        let t132 = t60 * t131;
        let t134 = t40 * t41 * t132;
        let tzk0 = t134 / 3.0;
        zk[ip] += tzk0;
    }
}
