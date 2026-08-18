//! GGA_K_APBE fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_apbe_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t24 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t34 = rho[ip] * rho[ip];
        let t40 = param_kappa + param_mu * t24 * t29 * sigma[ip] * t32 / t22 / t34 / 24.0;
        let t45 = 1.0 + param_kappa * (1.0 - param_kappa / t40);
        let t49 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t45);
        let tzk0 = 2.0 * t49;
        zk[ip] += tzk0;
        let t55 = t34 * rho[ip];
        let t58 = param_kappa * param_kappa;
        let t60 = t7 * t20 / t55 * t58;
        let t61 = t40 * t40;
        let t63 = 1.0 / t61 * param_mu;
        let t66 = t29 * sigma[ip] * t32;
        let t67 = t63 * t24 * t66;
        let t71 = piecewise3(t2, 0.0, t7 * t20 / t21 * t45 / 10.0 - t60 * t67 / 60.0);
        let tvrho0 = 2.0 * rho[ip] * t71 + 2.0 * t49;
        vrho[ip] += tvrho0;
        let t79 = t24 * t29 * t32;
        let t80 = t63 * t79;
        let t83 = piecewise3(t2, 0.0, t7 * t20 / t34 * t58 * t80 / 160.0);
        let tvsigma0 = 2.0 * rho[ip] * t83;
        vsigma[ip] += tvsigma0;
        let t92 = t34 * t34;
        let t96 = t7 * t20 / t92 * t58;
        let t99 = t92 * t34;
        let t104 = t7 * t20 / t22 / t99 * t58;
        let t107 = param_mu * param_mu;
        let t108 = 1.0 / t61 / t40 * t107;
        let t109 = t24 * t24;
        let t110 = t108 * t109;
        let t112 = 1.0 / t27 / t26;
        let t113 = sigma[ip] * sigma[ip];
        let t116 = t110 * t112 * t113 * t31;
        let t120 = piecewise3(t2, 0.0, -t7 * t20 / t21 / rho[ip] * t45 / 30.0 + 7.0 / 180.0 * t96 * t67 - t104 * t116 / 135.0);
        let tv2rho20 = 2.0 * rho[ip] * t120 + 4.0 * t71;
        v2rho2[ip] += tv2rho20;
        let t125 = t92 * rho[ip];
        let t130 = t7 * t20 / t22 / t125 * t58;
        let t133 = t110 * t112 * t31 * sigma[ip];
        let t137 = piecewise3(t2, 0.0, -t60 * t80 / 80.0 + t130 * t133 / 360.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t137 + 2.0 * t83;
        v2rhosigma[ip] += tv2rhosigma0;
        let t147 = t108 * t109 * t112 * t31;
        let t150 = piecewise3(t2, 0.0, -t7 * t20 / t22 / t92 * t58 * t147 / 960.0);
        let tv2sigma20 = 2.0 * rho[ip] * t150;
        v2sigma2[ip] += tv2sigma20;
    }
}
