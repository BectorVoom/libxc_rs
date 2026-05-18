//! GGA_X_HERMAN lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_herman.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_herman_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3::<f64>(rho[ip]);
        let t20 = t3 * t3;
        let t22 = pow_1_3::<f64>(1.0 / M_PI);
        let t23 = 1.0 / t22;
        let t25 = M_CBRT4;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t37 = 1.0 + 0.66666666666666666668e-3 * t20 * t23 * t25 * sigma[ip] * t28 * t33;
        let t41 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
        let t47 = t30 * rho[ip];
        let t50 = t17 / t18 / t47;
        let t53 = t25 * sigma[ip] * t28;
        let t57 = piecewise3::<f64>(t2, 0.0, -t6 * t17 / t31 * t37 / 8.0 + 0.13655681265105913629e-2 * t50 * t23 * t53);
        let tvrho0 = 2.0 * rho[ip] * t57 + 2.0 * t41;
        vrho[ip] += tvrho0;
        let t64 = t23 * t25 * t28;
        let t67 = piecewise3::<f64>(t2, 0.0, -0.51208804744147176112e-3 * t17 / t18 / t30 * t64);
        let tvsigma0 = 2.0 * rho[ip] * t67;
        vsigma[ip] += tvsigma0;
        let t76 = t30 * t30;
        let t79 = t17 / t18 / t76;
        let t84 = piecewise3::<f64>(t2, 0.0, t6 * t17 / t31 / rho[ip] * t37 / 12.0 - 0.40967043795317740887e-2 * t79 * t23 * t53);
        let tv2rho20 = 2.0 * rho[ip] * t84 + 4.0 * t57;
        v2rho2[ip] += tv2rho20;
        let t89 = piecewise3::<f64>(t2, 0.0, 0.11948721106967674426e-2 * t50 * t64);
        let tv2rhosigma0 = 2.0 * rho[ip] * t89 + 2.0 * t67;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let t100 = t17 / t18 / t76 / rho[ip];
        let t105 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t33 * t37 + 0.17448926060968667415e-1 * t100 * t23 * t53);
        let tv3rho30 = 2.0 * rho[ip] * t105 + 6.0 * t84;
        v3rho3[ip] += tv3rho30;
        let t111 = piecewise3::<f64>(t2, 0.0, -0.39829070356558914753e-2 * t79 * t64);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t111 + 4.0 * t89;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let t129 = piecewise3::<f64>(t2, 0.0, 10.0 / 27.0 * t6 * t17 / t31 / t47 * t37 - 0.92555173019051192375e-1 * t17 / t18 / t76 / t30 * t23 * t53);
        let tv4rho40 = 2.0 * rho[ip] * t129 + 8.0 * t105;
        v4rho4[ip] += tv4rho40;
        let t135 = piecewise3::<f64>(t2, 0.0, 0.17259263821175529726e-1 * t100 * t64);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t135 + 6.0 * t111;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let tv4rho2sigma20 = 0.0;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = 0.0;
        v4rhosigma3[ip] += tv4rhosigma30;
        let tv4sigma40 = 0.0;
        v4sigma4[ip] += tv4sigma40;
    }
}
