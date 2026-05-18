//! GGA_X_2D_B86 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b86_vxc_pol(
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
        let t1 = rho0 <= dens_threshold;
        let t2 = f64::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5::<f64>(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = f64::sqrt(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = f64::sqrt(t17);
        let t22 = t21 * t17;
        let t23 = piecewise3::<f64>(t18, t20, t22);
        let t25 = M_SQRT2;
        let t26 = t3 * t23 * t25;
        let t27 = f64::sqrt(t4);
        let t28 = rho0 * rho0;
        let t29 = t28 * rho0;
        let t30 = 1.0 / t29;
        let t31 = sigma0 * t30;
        let t33 = 1.0 + 0.2105e-2 * t31;
        let t36 = 1.0 + 0.119e-3 * t31;
        let t37 = 1.0 / t36;
        let t38 = t27 * t33 * t37;
        let t41 = piecewise3::<f64>(t1, 0.0, -2.0 / 3.0 * t26 * t38);
        let t42 = rho1 <= dens_threshold;
        let t43 = -t14;
        let t45 = piecewise5::<f64>(t12, t9, t8, t13, t43 * t5);
        let t46 = 1.0 + t45;
        let t47 = t46 <= zeta_threshold;
        let t48 = f64::sqrt(t46);
        let t49 = t48 * t46;
        let t50 = piecewise3::<f64>(t47, t20, t49);
        let t52 = t3 * t50 * t25;
        let t53 = rho1 * rho1;
        let t54 = t53 * rho1;
        let t55 = 1.0 / t54;
        let t56 = sigma2 * t55;
        let t58 = 1.0 + 0.2105e-2 * t56;
        let t61 = 1.0 + 0.119e-3 * t56;
        let t62 = 1.0 / t61;
        let t63 = t27 * t58 * t62;
        let t66 = piecewise3::<f64>(t42, 0.0, -2.0 / 3.0 * t52 * t63);
        let tzk0 = t41 + t66;
        zk[ip] += tzk0;
        let t67 = t4 * t4;
        let t68 = 1.0 / t67;
        let t69 = t14 * t68;
        let t71 = piecewise5::<f64>(t8, 0.0, t12, 0.0, t5 - t69);
        let t74 = piecewise3::<f64>(t18, 0.0, 3.0 / 2.0 * t21 * t71);
        let t76 = t3 * t74 * t25;
        let t79 = 1.0 / t27;
        let t81 = t79 * t33 * t37;
        let t83 = t26 * t81 / 3.0;
        let t84 = t23 * t25;
        let t85 = t84 * t27;
        let t86 = t28 * t28;
        let t87 = 1.0 / t86;
        let t88 = sigma0 * t87;
        let t89 = t88 * t37;
        let t92 = t36 * t36;
        let t93 = 1.0 / t92;
        let t94 = t33 * t93;
        let t95 = t94 * t88;
        let t99 = piecewise3::<f64>(t1, 0.0, -2.0 / 3.0 * t76 * t38 - t83 + 0.23752381467360539681e-2 * t85 * t89 - 0.13427712088436599629e-3 * t85 * t95);
        let t100 = t43 * t68;
        let t102 = piecewise5::<f64>(t12, 0.0, t8, 0.0, -t5 - t100);
        let t105 = piecewise3::<f64>(t47, 0.0, 3.0 / 2.0 * t48 * t102);
        let t107 = t3 * t105 * t25;
        let t111 = t79 * t58 * t62;
        let t113 = t52 * t111 / 3.0;
        let t115 = piecewise3::<f64>(t42, 0.0, -2.0 / 3.0 * t107 * t63 - t113);
        let tvrho0 = t41 + t66 + t4 * (t99 + t115);
        vrho[ip * 2] += tvrho0;
        let t119 = piecewise5::<f64>(t8, 0.0, t12, 0.0, -t5 - t69);
        let t122 = piecewise3::<f64>(t18, 0.0, 3.0 / 2.0 * t21 * t119);
        let t124 = t3 * t122 * t25;
        let t128 = piecewise3::<f64>(t1, 0.0, -2.0 / 3.0 * t124 * t38 - t83);
        let t130 = piecewise5::<f64>(t12, 0.0, t8, 0.0, t5 - t100);
        let t133 = piecewise3::<f64>(t47, 0.0, 3.0 / 2.0 * t48 * t130);
        let t135 = t3 * t133 * t25;
        let t138 = t50 * t25;
        let t139 = t138 * t27;
        let t140 = t53 * t53;
        let t141 = 1.0 / t140;
        let t142 = sigma2 * t141;
        let t143 = t142 * t62;
        let t146 = t61 * t61;
        let t147 = 1.0 / t146;
        let t148 = t58 * t147;
        let t149 = t148 * t142;
        let t153 = piecewise3::<f64>(t42, 0.0, -2.0 / 3.0 * t135 * t63 - t113 + 0.23752381467360539681e-2 * t139 * t143 - 0.13427712088436599629e-3 * t139 * t149);
        let tvrho1 = t41 + t66 + t4 * (t128 + t153);
        vrho[ip * 2 + 1] += tvrho1;
        let t157 = t27 * t30 * t37;
        let t160 = t94 * t30;
        let t164 = piecewise3::<f64>(t1, 0.0, -0.79174604891201798933e-3 * t84 * t157 + 0.44759040294788665431e-4 * t85 * t160);
        let tvsigma0 = t4 * t164;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t166 = t27 * t55 * t62;
        let t169 = t148 * t55;
        let t173 = piecewise3::<f64>(t42, 0.0, -0.79174604891201798933e-3 * t138 * t166 + 0.44759040294788665431e-4 * t139 * t169);
        let tvsigma2 = t4 * t173;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
