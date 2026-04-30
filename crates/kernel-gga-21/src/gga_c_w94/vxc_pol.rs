//! GGA_C_W94 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 34 shared lines across all orders.
//! Delta: 31 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_w94_vxc_pol(
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
        // --- shared preamble (34 lines) ---
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 0.0 < t4;
        let t6 = piecewise3(t5, t4, -t4);
        let t7 = 0.1e-9 < t6;
        let t8 = piecewise3(t7, t6, 0.1e-9);
        let t9 = pow_1_3(t8);
        let t10 = t9 * t9;
        let t12 = -t10 * t8 + 1.0;
        let t13 = f64::sqrt(t12);
        let t15 = sigma0 + 2.0 * sigma1 + sigma2;
        let t16 = f64::sqrt(t15);
        let t17 = t16 * t15;
        let t18 = t2 * t2;
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t22 = pow_1_3(t2);
        let t24 = 1.0 / t22 / t2;
        let t25 = t16 * t24;
        let t26 = f64::powf(t25, 1.0 / 16.0);
        let t27 = t26 * t26;
        let t28 = t27 * t26;
        let t31 = t18 * t2;
        let t32 = 1.0 / t31;
        let t35 = M_CBRT3;
        let t37 = pow_1_3(1.0 / M_PI);
        let t38 = t35 * t37;
        let t39 = M_CBRT4;
        let t40 = t39 * t39;
        let t45 = 0.118e2 + 0.15067e0 * t28 * t17 * t20 + 0.1102e-1 * t15 * t32 + t38 * t40 / t22 / 4.0;
        let t46 = 1.0 / t45;
        let tzk0 = -t13 * t46;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (31 lines) ---
        let t48 = 1.0 / t13;
        let t49 = t2 * t48;
        let t50 = t46 * t10;
        let t51 = 1.0 / t18;
        let t52 = t1 * t51;
        let t53 = t3 - t52;
        let t55 = piecewise3(t5, t53, -t53);
        let t56 = piecewise3(t7, t55, 0.0);
        let t60 = t2 * t13;
        let t61 = t45 * t45;
        let t62 = 1.0 / t61;
        let t63 = t22 * t22;
        let t65 = 1.0 / t63 / t18;
        let t67 = t28 * t15 * t65;
        let t68 = t67 * t16;
        let t70 = 1.0 / t22 / t18;
        let t78 = -0.6403475e0 * t68 * t70 - 0.3306e-1 * t15 * t20 - t38 * t40 * t24 / 12.0;
        let t80 = t60 * t62 * t78;
        let tvrho0 = tzk0 + 5.0 / 6.0 * t49 * t50 * t56 + t80;
        vrho[ip * 2] += tvrho0;
        let t81 = -t3 - t52;
        let t83 = piecewise3(t5, t81, -t81);
        let t84 = piecewise3(t7, t83, 0.0);
        let tvrho1 = tzk0 + 5.0 / 6.0 * t49 * t50 * t84 + t80;
        vrho[ip * 2 + 1] += tvrho1;
        let t88 = 1.0 / t16;
        let t89 = t67 * t88;
        let t90 = t89 * t24;
        let t93 = 0.2401303125e0 * t90 + 0.1102e-1 * t32;
        let tvsigma0 = t60 * t62 * t93;
        vsigma[ip * 3] += tvsigma0;
        let t97 = 0.480260625e0 * t90 + 0.2204e-1 * t32;
        let tvsigma1 = t60 * t62 * t97;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
