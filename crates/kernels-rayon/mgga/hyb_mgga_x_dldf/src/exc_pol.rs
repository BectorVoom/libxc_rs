//! HYB_MGGA_X_DLDF exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_dldf_exc_pol(
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
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = pow_1_3(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = pow_1_3(t17);
        let t23 = piecewise3(t18, t20, t21 * t17);
        let t24 = t3 * t23;
        let t25 = pow_1_3(t4);
        let t26 = M_CBRT6;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3(t27);
        let t29 = t28 * t28;
        let t30 = 1.0 / t29;
        let t31 = t26 * t30;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t40 = 4.8827323 + 0.0146297 * t31 * sigma0 * t36;
        let t43 = 5.8827323 - 23.84107471346329 / t40;
        let t44 = t25 * t43;
        let t45 = t26 * t26;
        let t47 = 3.0 / 10.0 * t45 * t29;
        let t49 = 1.0 / t34 / rho0;
        let t50 = tau0 * t49;
        let t51 = t47 - t50;
        let t52 = t47 + t50;
        let t53 = 1.0 / t52;
        let t56 = t51 * t51;
        let t57 = t52 * t52;
        let t58 = 1.0 / t57;
        let t61 = t56 * t51;
        let t62 = t57 * t52;
        let t63 = 1.0 / t62;
        let t66 = t56 * t56;
        let t67 = t57 * t57;
        let t68 = 1.0 / t67;
        let t71 = 1.0 - 0.1637571 * t51 * t53 - 0.1880028 * t56 * t58 - 0.4490609 * t61 * t63 - 0.0082359 * t66 * t68;
        let t72 = t44 * t71;
        let t75 = piecewise3(t2, 0.0, -0.09872727257880975 * t24 * t72);
        let t76 = rho1 <= dens_threshold;
        let t77 = -t14;
        let t79 = piecewise5(t12, t9, t8, t13, t77 * t5);
        let t80 = 1.0 + t79;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t20, t82 * t80);
        let t85 = t3 * t84;
        let t86 = rho1 * rho1;
        let t87 = pow_1_3(rho1);
        let t88 = t87 * t87;
        let t90 = 1.0 / t88 / t86;
        let t94 = 4.8827323 + 0.0146297 * t31 * sigma2 * t90;
        let t97 = 5.8827323 - 23.84107471346329 / t94;
        let t98 = t25 * t97;
        let t100 = 1.0 / t88 / rho1;
        let t101 = tau1 * t100;
        let t102 = t47 - t101;
        let t103 = t47 + t101;
        let t104 = 1.0 / t103;
        let t107 = t102 * t102;
        let t108 = t103 * t103;
        let t109 = 1.0 / t108;
        let t112 = t107 * t102;
        let t113 = t108 * t103;
        let t114 = 1.0 / t113;
        let t117 = t107 * t107;
        let t118 = t108 * t108;
        let t119 = 1.0 / t118;
        let t122 = 1.0 - 0.1637571 * t102 * t104 - 0.1880028 * t107 * t109 - 0.4490609 * t112 * t114 - 0.0082359 * t117 * t119;
        let t123 = t98 * t122;
        let t126 = piecewise3(t76, 0.0, -0.09872727257880975 * t85 * t123);
        let tzk0 = t75 + t126;
        zk[ip] += tzk0;
    }
}
