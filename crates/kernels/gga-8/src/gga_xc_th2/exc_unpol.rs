//! GGA_XC_TH2 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 51 shared lines across all orders.
//! Delta: 51 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_2_3, pow_4_3, pow_5_3, pow_7_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_xc_th2_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (51 lines) ---
        let t1 = f64::powf(2.0, 1.0 / 12.0);
        let t2 = t1 * t1;
        let t3 = t2 * t1;
        let t4 = t2 * t2;
        let t5 = t4 * t4;
        let t6 = t5 * t3;
        let t7 = f64::powf(rho[ip], 1.0 / 12.0);
        let t11 = f64::powf(2.0, 1.0 / 6.0);
        let t12 = t11 * t11;
        let t13 = t12 * t12;
        let t14 = t13 * t11;
        let t15 = f64::powf(rho[ip], 1.0 / 6.0);
        let t16 = t15 * rho[ip];
        let t19 = M_CBRT2;
        let t20 = t19 * t19;
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * rho[ip];
        let t25 = M_SQRT2;
        let t26 = f64::sqrt(rho[ip]);
        let t27 = t26 * rho[ip];
        let t30 = t21 * t21;
        let t31 = t30 * rho[ip];
        let t32 = t19 * t31;
        let t34 = t4 * t3;
        let t35 = t34 * t7;
        let t36 = f64::sqrt(sigma[ip]);
        let t38 = pow_1_3(zeta_threshold);
        let t40 = piecewise3(1.0 <= zeta_threshold, t38 * zeta_threshold, 1.0);
        let t41 = t36 * t40;
        let t44 = t25 * t15;
        let t47 = t19 * t21;
        let t50 = t11 * t26;
        let t53 = 1.0 / rho[ip];
        let t54 = t19 * t53;
        let t55 = t40 * t40;
        let t56 = sigma[ip] * t55;
        let t59 = t15 * t15;
        let t60 = t59 * t59;
        let t61 = t60 * t15;
        let t62 = 1.0 / t61;
        let t63 = t11 * t62;
        let t66 = 1.0 / t30;
        let t70 = rho[ip] * rho[ip];
        let t72 = 1.0 / t30 / t70;
        let t73 = sigma[ip] * t72;
        let t74 = t73 * t55;
        let t75 = t74 - t73;
        let t78 = t61 * rho[ip];
        let t79 = t11 * t78;
        let t84 = 0.3394155e0 * t6 * t7 * rho[ip] - 0.879105e0 * t14 * t16 + 0.63838e0 * t20 * t22 - 0.803945e0 * t25 * t27 + 0.182805e0 * t32 - 0.4533175e-1 * t35 * t41 + 0.3674325e-1 * t44 * t41 + 0.3678525e-1 * t47 * t41 - 0.17922925e-1 * t50 * t41 - 0.50895875e-2 * t54 * t56 + 0.26828125e-2 * t63 * t56 - 0.960195e-4 * t66 * sigma[ip] * t55 + 0.1551885e-1 * t32 * t75 - 0.360163e-1 * t79 * t75 + 0.223281e-1 * t70 * t75;
        let tzk0 = t84 * t53;
        zk[ip] += tzk0;
    }
}
