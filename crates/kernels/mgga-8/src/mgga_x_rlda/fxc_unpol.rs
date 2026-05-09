//! MGGA_X_RLDA fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 26 shared lines across all orders.
//! Delta: 32 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_rlda_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    param_prefactor: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (26 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t5 = t4 * t4;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = t5 * t16;
        let t18 = pow_1_3(rho[ip]);
        let t21 = pow_1_3(1.0 / M_PI);
        let t22 = 1.0 / t21;
        let t23 = param_prefactor * t22;
        let t24 = M_CBRT4;
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = tau[ip] * t26;
        let t28 = t18 * t18;
        let t30 = 1.0 / t28 / rho[ip];
        let t33 = lapl[ip] * t26;
        let t36 = 2.0 * t27 * t30 - t33 * t30 / 4.0;
        let t39 = t23 * t24 / t36;
        let t42 = piecewise3(t3, 0.0, -15.0 / 16.0 * t17 * t18 * t39);
        let tzk0 = 2.0 * t42;
        zk[ip] += tzk0;
        // --- vxc delta (20 lines) ---
        let t43 = 1.0 / t28;
        let t48 = t17 * t18 * param_prefactor;
        let t49 = t22 * t24;
        let t50 = t36 * t36;
        let t51 = 1.0 / t50;
        let t52 = rho[ip] * rho[ip];
        let t54 = 1.0 / t28 / t52;
        let t59 = -10.0 / 3.0 * t27 * t54 + 5.0 / 12.0 * t33 * t54;
        let t61 = t49 * t51 * t59;
        let t65 = piecewise3(t3, 0.0, -5.0 / 16.0 * t17 * t43 * t39 + 15.0 / 16.0 * t48 * t61);
        let tvrho0 = 2.0 * rho[ip] * t65 + 2.0 * t42;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t70 = 1.0 / t18 / rho[ip] * param_prefactor;
        let t71 = t17 * t70;
        let t73 = t49 * t51 * t26;
        let t74 = t71 * t73;
        let t76 = piecewise3(t3, 0.0, -15.0 / 64.0 * t74);
        let tvlapl0 = 2.0 * rho[ip] * t76;
        vlapl[ip] += tvlapl0;
        let t79 = piecewise3(t3, 0.0, 15.0 / 8.0 * t74);
        let tvtau0 = 2.0 * rho[ip] * t79;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (32 lines) ---
        let t86 = t17 * t43 * param_prefactor;
        let t90 = 1.0 / t50 / t36;
        let t91 = t59 * t59;
        let t93 = t49 * t90 * t91;
        let t96 = t52 * rho[ip];
        let t98 = 1.0 / t28 / t96;
        let t103 = 80.0 / 9.0 * t27 * t98 - 10.0 / 9.0 * t33 * t98;
        let t105 = t49 * t51 * t103;
        let t109 = piecewise3(t3, 0.0, 5.0 / 24.0 * t17 * t30 * t39 + 5.0 / 8.0 * t86 * t61 - 15.0 / 8.0 * t48 * t93 + 15.0 / 16.0 * t48 * t105);
        let tv2rho20 = 2.0 * rho[ip] * t109 + 4.0 * t65;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t115 = t17 / t18 / t52 * param_prefactor;
        let t116 = t115 * t73;
        let t118 = t90 * t26;
        let t120 = t49 * t118 * t59;
        let t121 = t71 * t120;
        let t124 = piecewise3(t3, 0.0, 5.0 / 16.0 * t116 + 15.0 / 32.0 * t121);
        let tv2rholapl0 = 2.0 * rho[ip] * t124 + 2.0 * t76;
        v2rholapl[ip] += tv2rholapl0;
        let t130 = piecewise3(t3, 0.0, -5.0 / 2.0 * t116 - 15.0 / 4.0 * t121);
        let tv2rhotau0 = 2.0 * rho[ip] * t130 + 2.0 * t79;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t135 = t17 / t96 * param_prefactor;
        let t137 = t49 * t90 * t25;
        let t138 = t135 * t137;
        let t140 = piecewise3(t3, 0.0, -15.0 / 64.0 * t138);
        let tv2lapl20 = 2.0 * rho[ip] * t140;
        v2lapl2[ip] += tv2lapl20;
        let t143 = piecewise3(t3, 0.0, 15.0 / 8.0 * t138);
        let tv2lapltau0 = 2.0 * rho[ip] * t143;
        v2lapltau[ip] += tv2lapltau0;
        let t146 = piecewise3(t3, 0.0, -15.0 * t138);
        let tv2tau20 = 2.0 * rho[ip] * t146;
        v2tau2[ip] += tv2tau20;
    }
}
