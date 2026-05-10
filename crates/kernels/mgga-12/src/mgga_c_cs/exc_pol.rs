//! MGGA_C_CS exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 57 shared lines across all orders.
//! Delta: 57 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_cs_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        // --- shared preamble (57 lines) ---
        let t2 = rho0 - rho1;
        let t3 = t2 * t2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = 1.0 / t5;
        let t8 = -t3 * t6 + 1.0;
        let t9 = pow_1_3(t4);
        let t10 = 1.0 / t9;
        let t12 = 1.0 + 0.34899999999999999998e0 * t10;
        let t13 = 1.0 / t12;
        let t14 = t8 * t13;
        let t16 = f64::exp(-0.2533e0 * t10);
        let t17 = 1.0 / t4;
        let t18 = t2 * t17;
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = zeta_threshold * zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * t21;
        let t25 = t19 * t19;
        let t26 = pow_1_3(t19);
        let t27 = t26 * t26;
        let t29 = piecewise3(t20, t24, t27 * t25);
        let t30 = M_CBRT2;
        let t31 = t29 * t30;
        let t32 = pow_1_3(rho0);
        let t33 = t32 * t32;
        let t35 = 1.0 / t33 / rho0;
        let t37 = lapl0 * t35;
        let t39 = tau0 * t35 - t37 / 8.0;
        let t41 = 1.0 - t18;
        let t42 = t41 <= zeta_threshold;
        let t43 = t41 * t41;
        let t44 = pow_1_3(t41);
        let t45 = t44 * t44;
        let t47 = piecewise3(t42, t24, t45 * t43);
        let t48 = t47 * t30;
        let t49 = pow_1_3(rho1);
        let t50 = t49 * t49;
        let t52 = 1.0 / t50 / rho1;
        let t54 = lapl1 * t52;
        let t56 = tau1 * t52 - t54 / 8.0;
        let t59 = sigma0 + 2.0 * sigma1 + sigma2;
        let t60 = t9 * t9;
        let t62 = 1.0 / t60 / t5;
        let t64 = t19 / 2.0;
        let t65 = pow_1_3(t64);
        let t66 = t65 * t65;
        let t67 = t66 * t64;
        let t69 = t41 / 2.0;
        let t70 = pow_1_3(t69);
        let t71 = t70 * t70;
        let t72 = t71 * t69;
        let t75 = t31 * t39 / 8.0 + t37 * t67 / 8.0 + t48 * t56 / 8.0 + t54 * t72 / 8.0 - t59 * t62 / 8.0;
        let t78 = 1.0 + 0.264e0 * t16 * t75;
        let tzk0 = -0.4918e-1 * t14 * t78;
        zk[ip] += tzk0;
    }
}
