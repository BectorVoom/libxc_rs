//! GGA_C_TCA vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_tca.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_tca_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        let t23 = pow_1_3(1.0 / M_PI);
        let t24 = t21 * t23;
        let t25 = M_CBRT4;
        let t26 = t25 * t25;
        let t27 = pow_1_3(t2);
        let t32 = 0.488827e1 + 0.79425925e0 * t24 * t26 / t27;
        let t33 = f64::atan(t32);
        let t35 = -0.655868e0 * t33 + 0.897889e0;
        let t36 = t20 * t35;
        let t37 = t21 * t21;
        let t38 = t36 * t37;
        let t39 = 1.0 / t23;
        let t40 = t39 * t25;
        let t41 = M_CBRT6;
        let t42 = t41 * t41;
        let t43 = M_PI * M_PI;
        let t44 = pow_1_3(t43);
        let t45 = 1.0 / t44;
        let t46 = t42 * t45;
        let t47 = M_CBRT2;
        let t49 = sigma0 + 2.0 * sigma1 + sigma2;
        let t50 = f64::sqrt(t49);
        let t51 = t47 * t50;
        let t52 = t27 * t2;
        let t53 = 1.0 / t52;
        let t55 = t46 * t51 * t53;
        let t56 = f64::powf(t55, 0.23e1);
        let t58 = 1.0 + 0.47121507034422759993e-2 * t56;
        let t59 = 1.0 / t58;
        let t62 = t38 * t40 * t27 * t59;
        let tzk0 = t62 / 3.0;
        zk[ip] += tzk0;
        let t63 = 4.0 / 9.0 * t62;
        let t65 = t35 * t37;
        let t66 = t52 * t19 * t65;
        let t67 = 1.0 / t9;
        let t68 = t2 * t2;
        let t69 = 1.0 / t68;
        let t70 = t1 * t69;
        let t71 = t3 - t70;
        let t74 = piecewise3(t6, 0.0, 2.0 / 3.0 * t67 * t71);
        let t75 = 1.0 / t14;
        let t76 = -t71;
        let t79 = piecewise3(t13, 0.0, 2.0 / 3.0 * t75 * t76);
        let t81 = t74 / 2.0 + t79 / 2.0;
        let t82 = t59 * t81;
        let t85 = t32 * t32;
        let t86 = t85 + 1.0;
        let t87 = 1.0 / t86;
        let t88 = t20 * t87;
        let t90 = 0.69457230103866666663e0 * t88 * t59;
        let t91 = t3 * t20;
        let t93 = t37 * t39;
        let t94 = t93 * t25;
        let t96 = t58 * t58;
        let t97 = 1.0 / t96;
        let t98 = f64::powf(t55, 0.13e1);
        let t99 = t97 * t98;
        let t100 = t99 * t42;
        let t101 = t45 * t47;
        let t102 = t101 * t50;
        let t103 = t100 * t102;
        let t105 = 0.48168651635187710217e-2 * t91 * t35 * t94 * t103;
        let tvrho0 = t66 * t40 * t82 + t105 + t63 + t90;
        vrho[ip * 2] += tvrho0;
        let t106 = -t3 - t70;
        let t109 = piecewise3(t6, 0.0, 2.0 / 3.0 * t67 * t106);
        let t110 = -t106;
        let t113 = piecewise3(t13, 0.0, 2.0 / 3.0 * t75 * t110);
        let t115 = t109 / 2.0 + t113 / 2.0;
        let t116 = t59 * t115;
        let t117 = t40 * t116;
        let tvrho1 = t66 * t117 + t105 + t63 + t90;
        vrho[ip * 2 + 1] += tvrho1;
        let t119 = t36 * t94;
        let t120 = 1.0 / t50;
        let t121 = t101 * t120;
        let t122 = t100 * t121;
        let t123 = t119 * t122;
        let tvsigma0 = -0.18063244363195391331e-2 * t123;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -0.3612648872639078266e-2 * t123;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
