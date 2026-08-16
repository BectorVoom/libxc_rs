//! GGA_K_APBE lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_apbe_lxc_unpol(
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
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3::<f64>(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3::<f64>(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3::<f64>(rho[ip]);
        let t22 = t21 * t21;
        let t24 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3::<f64>(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t34 = rho[ip] * rho[ip];
        let t40 = param_kappa + param_mu * t24 * t29 * sigma[ip] * t32 / t22 / t34 / 24.0;
        let t45 = 1.0 + param_kappa * (1.0 - param_kappa / t40);
        let t49 = piecewise3::<f64>(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t45);
        let tzk0 = 2.0 * t49;
        zk[ip] += tzk0;
        let t55 = t34 * rho[ip];
        let t58 = param_kappa * param_kappa;
        let t60 = t7 * t20 / t55 * t58;
        let t61 = t40 * t40;
        let t63 = 1.0 / t61 * param_mu;
        let t66 = t29 * sigma[ip] * t32;
        let t67 = t63 * t24 * t66;
        let t71 = piecewise3::<f64>(t2, 0.0, t7 * t20 / t21 * t45 / 10.0 - t60 * t67 / 60.0);
        let tvrho0 = 2.0 * rho[ip] * t71 + 2.0 * t49;
        vrho[ip] += tvrho0;
        let t79 = t24 * t29 * t32;
        let t80 = t63 * t79;
        let t83 = piecewise3::<f64>(t2, 0.0, t7 * t20 / t34 * t58 * t80 / 160.0);
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
        let t120 = piecewise3::<f64>(t2, 0.0, -t7 * t20 / t21 / rho[ip] * t45 / 30.0 + 7.0 / 180.0 * t96 * t67 - t104 * t116 / 135.0);
        let tv2rho20 = 2.0 * rho[ip] * t120 + 4.0 * t71;
        v2rho2[ip] += tv2rho20;
        let t125 = t92 * rho[ip];
        let t130 = t7 * t20 / t22 / t125 * t58;
        let t133 = t110 * t112 * t31 * sigma[ip];
        let t137 = piecewise3::<f64>(t2, 0.0, -t60 * t80 / 80.0 + t130 * t133 / 360.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t137 + 2.0 * t83;
        v2rhosigma[ip] += tv2rhosigma0;
        let t147 = t108 * t109 * t112 * t31;
        let t150 = piecewise3::<f64>(t2, 0.0, -t7 * t20 / t22 / t92 * t58 * t147 / 960.0);
        let tv2sigma20 = 2.0 * rho[ip] * t150;
        v2sigma2[ip] += tv2sigma20;
        let t162 = t7 * t20 / t125 * t58;
        let t165 = t92 * t55;
        let t170 = t7 * t20 / t22 / t165 * t58;
        let t173 = t5 * t5;
        let t176 = t4 / t173 / t26;
        let t177 = t92 * t92;
        let t178 = t177 * t34;
        let t182 = t176 * t20 / t21 / t178;
        let t183 = t61 * t61;
        let t184 = 1.0 / t183;
        let t185 = t58 * t184;
        let t186 = t107 * param_mu;
        let t187 = t113 * sigma[ip];
        let t189 = t185 * t186 * t187;
        let t193 = piecewise3::<f64>(t2, 0.0, 2.0 / 45.0 * t7 * t20 / t21 / t34 * t45 - 41.0 / 270.0 * t162 * t67 + t170 * t116 / 15.0 - 4.0 / 135.0 * t182 * t189);
        let tv3rho30 = 2.0 * rho[ip] * t193 + 6.0 * t120;
        v3rho3[ip] += tv3rho30;
        let t205 = t176 * t20 / t21 / t177 / rho[ip];
        let t207 = t185 * t186 * t113;
        let t211 = piecewise3::<f64>(t2, 0.0, 3.0 / 80.0 * t96 * t80 - 23.0 / 1080.0 * t104 * t133 + t205 * t207 / 90.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t211 + 4.0 * t137;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t217 = 1.0 / t21 / t177;
        let t221 = t185 * t186 * sigma[ip];
        let t225 = piecewise3::<f64>(t2, 0.0, 7.0 / 1440.0 * t130 * t147 - t176 * t20 * t217 * t221 / 240.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t225 + 2.0 * t150;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t228 = t176 * t20;
        let t232 = t184 * t186;
        let t236 = piecewise3::<f64>(t2, 0.0, t228 / t21 / t165 * t58 * t232 / 640.0);
        let tv3sigma30 = 2.0 * rho[ip] * t236;
        v3sigma3[ip] += tv3sigma30;
        let t258 = t177 * t55;
        let t272 = t107 * t107;
        let t273 = 1.0 / t183 / t40 * t272;
        let t274 = t113 * t113;
        let t280 = piecewise3::<f64>(t2, 0.0, -14.0 / 135.0 * t7 * t20 / t21 / t55 * t45 + 611.0 / 810.0 * t7 * t20 / t99 * t58 * t67 - 703.0 / 1215.0 * t7 * t20 / t22 / t177 * t58 * t116 + 232.0 / 405.0 * t176 * t20 / t21 / t258 * t189 - 16.0 / 1215.0 * t176 * t20 / t177 / t99 * t58 * t273 * t274 * t79);
        let tv4rho40 = 2.0 * rho[ip] * t280 + 8.0 * t193;
        v4rho4[ip] += tv4rho40;
        let t300 = piecewise3::<f64>(t2, 0.0, -3.0 / 20.0 * t162 * t80 + 257.0 / 1620.0 * t170 * t133 - 17.0 / 90.0 * t182 * t207 + 2.0 / 405.0 * t176 * t20 / t177 / t125 * t58 * t273 * t187 * t79);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t300 + 6.0 * t211;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t318 = piecewise3::<f64>(t2, 0.0, -119.0 / 4320.0 * t104 * t147 + 13.0 / 240.0 * t205 * t221 - t176 * t20 / t177 / t92 * t58 * t273 * t113 * t79 / 540.0);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t318 + 4.0 * t225;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t334 = piecewise3::<f64>(t2, 0.0, -11.0 / 960.0 * t228 * t217 * t58 * t232 + t176 * t20 / t258 * t58 * t273 * t24 * t66 / 1440.0);
        let tv4rhosigma30 = 2.0 * rho[ip] * t334 + 2.0 * t236;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t344 = piecewise3::<f64>(t2, 0.0, -t176 * t20 / t178 * t58 * t273 * t79 / 3840.0);
        let tv4sigma40 = 2.0 * rho[ip] * t344;
        v4sigma4[ip] += tv4sigma40;
    }
}
