//! GGA_X_LAG fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lag.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lag_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = t3 * t14;
        let t16 = pow_1_3(rho[ip]);
        let t17 = M_CBRT6;
        let t18 = t17 * t17;
        let t19 = M_PI * M_PI;
        let t20 = pow_1_3(t19);
        let t21 = 1.0 / t20;
        let t22 = t18 * t21;
        let t23 = f64::sqrt(sigma[ip]);
        let t24 = M_CBRT2;
        let t29 = t22 * t23 * t24 / t16 / rho[ip];
        let t30 = f64::powf(t29, 2.626712);
        let t33 = 1.0 + 0.00013471619689594795 * t30;
        let t34 = f64::powf(t33, -0.657946);
        let t38 = piecewise3(t2, 0.0, -1.540002877192757e-05 * t15 * t16 * t30 * t34);
        let tzk0 = 2.0 * t38;
        zk[ip] += tzk0;
        let t39 = t16 * t16;
        let t45 = rho[ip] * rho[ip];
        let t46 = 1.0 / t45;
        let t47 = f64::powf(t29, 1.626712);
        let t49 = t15 * t46 * t47;
        let t50 = t34 * t18;
        let t52 = t21 * t23 * t24;
        let t53 = t50 * t52;
        let t56 = f64::powf(t29, 4.253424);
        let t58 = t15 * t46 * t56;
        let t59 = f64::powf(t33, -1.657946);
        let t60 = t59 * t18;
        let t61 = t60 * t52;
        let t65 = piecewise3(t2, 0.0, -5.133342923975857e-06 * t15 / t39 * t30 * t34 + 5.393525383408988e-05 * t49 * t53 - 4.780604235623332e-09 * t58 * t61);
        let tvrho0 = 2.0 * rho[ip] * t65 + 2.0 * t38;
        vrho[ip] += tvrho0;
        let t68 = 1.0 / rho[ip];
        let t70 = t15 * t68 * t47;
        let t71 = 1.0 / t23;
        let t73 = t21 * t71 * t24;
        let t74 = t50 * t73;
        let t78 = t15 * t68 * t56;
        let t79 = t60 * t73;
        let t83 = piecewise3(t2, 0.0, -2.0225720187783704e-05 * t70 * t74 + 1.7927265883587494e-09 * t78 * t79);
        let tvsigma0 = 2.0 * rho[ip] * t83;
        vsigma[ip] += tvsigma0;
        let t92 = t45 * rho[ip];
        let t93 = 1.0 / t92;
        let t95 = t15 * t93 * t47;
        let t99 = t15 * t93 * t56;
        let t102 = t45 * t45;
        let t104 = 1.0 / t16 / t102;
        let t105 = f64::powf(t29, 0.626712);
        let t107 = t15 * t104 * t105;
        let t108 = t34 * t17;
        let t109 = t20 * t20;
        let t110 = 1.0 / t109;
        let t112 = t24 * t24;
        let t113 = t110 * sigma[ip] * t112;
        let t114 = t108 * t113;
        let t117 = f64::powf(t29, 3.253424);
        let t119 = t15 * t104 * t117;
        let t120 = t59 * t17;
        let t121 = t120 * t113;
        let t124 = f64::powf(t29, 5.880136);
        let t126 = t15 * t104 * t124;
        let t127 = f64::powf(t33, -2.657946);
        let t128 = t127 * t17;
        let t129 = t128 * t113;
        let t133 = piecewise3(t2, 0.0, 3.4222286159839043e-06 * t15 / t39 / rho[ip] * t30 * t34 - 8.989208972348313e-05 * t95 * t53 + 7.967673726038885e-09 * t99 * t61 - 0.0007018969970796801 * t107 * t114 + 2.631296584261165e-07 * t119 * t121 - 2.2437549929142988e-11 * t126 * t129);
        let tv2rho20 = 2.0 * rho[ip] * t133 + 4.0 * t65;
        v2rho2[ip] += tv2rho20;
        let t139 = 1.0 / t16 / t92;
        let t141 = t15 * t139 * t105;
        let t142 = t110 * t112;
        let t143 = t108 * t142;
        let t147 = t15 * t139 * t117;
        let t148 = t120 * t142;
        let t154 = t15 * t139 * t124;
        let t155 = t128 * t142;
        let t159 = piecewise3(t2, 0.0, 2.0225720187783704e-05 * t49 * t74 + 0.00026321137390488005 * t141 * t143 - 9.86736219097937e-08 * t147 * t148 - 1.7927265883587494e-09 * t58 * t79 + 8.414081223428621e-12 * t154 * t155);
        let tv2rhosigma0 = 2.0 * rho[ip] * t159 + 2.0 * t83;
        v2rhosigma[ip] += tv2rhosigma0;
        let t163 = 1.0 / t16 / t45;
        let t165 = t15 * t163 * t105;
        let t166 = 1.0 / sigma[ip];
        let t168 = t110 * t166 * t112;
        let t169 = t108 * t168;
        let t173 = t15 * t163 * t117;
        let t174 = t120 * t168;
        let t177 = t23 * sigma[ip];
        let t178 = 1.0 / t177;
        let t180 = t21 * t178 * t24;
        let t181 = t50 * t180;
        let t185 = t15 * t163 * t124;
        let t186 = t128 * t168;
        let t189 = t60 * t180;
        let t193 = piecewise3(t2, 0.0, -9.870426521433003e-05 * t165 * t169 + 3.700260821617263e-08 * t173 * t174 + 1.0112860093891852e-05 * t70 * t181 - 3.1552804587857326e-12 * t185 * t186 - 8.963632941793747e-10 * t78 * t189);
        let tv2sigma20 = 2.0 * rho[ip] * t193;
        v2sigma2[ip] += tv2sigma20;
    }
}
