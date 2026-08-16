//! GGA_X_PW86 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pw86.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_pw86_lxc_unpol(
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
    param_aa: f64,
    param_bb: f64,
    param_cc: f64,
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
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = param_aa * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t37 = t20 * t20;
        let t38 = param_bb * t37;
        let t40 = 1.0 / t23 / t22;
        let t41 = t38 * t40;
        let t42 = sigma[ip] * sigma[ip];
        let t43 = t42 * t27;
        let t44 = t30 * t30;
        let t45 = t44 * rho[ip];
        let t47 = 1.0 / t18 / t45;
        let t51 = t22 * t22;
        let t53 = param_cc / t51;
        let t54 = t42 * sigma[ip];
        let t55 = t44 * t44;
        let t56 = 1.0 / t55;
        let t60 = 1.0 + t26 * t29 * t33 / 24.0 + t41 * t43 * t47 / 288.0 + t53 * t54 * t56 / 576.0;
        let t61 = f64::powf(t60, 1.0 / 15.0);
        let t65 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t61);
        let tzk0 = 2.0 * t65;
        zk[ip] += tzk0;
        let t66 = 1.0 / t31;
        let t71 = t6 * t17;
        let t72 = t61 * t61;
        let t73 = t72 * t72;
        let t75 = t73 * t73;
        let t76 = t75 * t73 * t72;
        let t77 = 1.0 / t76;
        let t78 = t18 * t77;
        let t79 = t30 * rho[ip];
        let t81 = 1.0 / t31 / t79;
        let t85 = t44 * t30;
        let t87 = 1.0 / t18 / t85;
        let t91 = t55 * rho[ip];
        let t92 = 1.0 / t91;
        let t96 = -t26 * t29 * t81 / 9.0 - t41 * t43 * t87 / 54.0 - t53 * t54 * t92 / 72.0;
        let t101 = piecewise3::<f64>(t2, 0.0, -t6 * t17 * t66 * t61 / 8.0 - t71 * t78 * t96 / 40.0);
        let tvrho0 = 2.0 * rho[ip] * t101 + 2.0 * t65;
        vrho[ip] += tvrho0;
        let t104 = t25 * t28;
        let t108 = sigma[ip] * t27;
        let t115 = t21 * t104 * t33 / 24.0 + t41 * t108 * t47 / 144.0 + t53 * t42 * t56 / 192.0;
        let t119 = piecewise3::<f64>(t2, 0.0, -t71 * t78 * t115 / 40.0);
        let tvsigma0 = 2.0 * rho[ip] * t119;
        vsigma[ip] += tvsigma0;
        let t123 = 1.0 / t31 / rho[ip];
        let t128 = t66 * t77;
        let t133 = 1.0 / t76 / t60;
        let t134 = t18 * t133;
        let t135 = t96 * t96;
        let t140 = 1.0 / t31 / t44;
        let t144 = t44 * t79;
        let t146 = 1.0 / t18 / t144;
        let t151 = 1.0 / t55 / t30;
        let t155 = 11.0 / 27.0 * t26 * t29 * t140 + 19.0 / 162.0 * t41 * t43 * t146 + t53 * t54 * t151 / 8.0;
        let t160 = piecewise3::<f64>(t2, 0.0, t6 * t17 * t123 * t61 / 12.0 - t71 * t128 * t96 / 60.0 + 7.0 / 300.0 * t71 * t134 * t135 - t71 * t78 * t155 / 40.0);
        let tv2rho20 = 2.0 * rho[ip] * t160 + 4.0 * t101;
        v2rho2[ip] += tv2rho20;
        let t166 = t115 * t96;
        let t179 = -t21 * t104 * t81 / 9.0 - t41 * t108 * t87 / 27.0 - t53 * t42 * t92 / 24.0;
        let t184 = piecewise3::<f64>(t2, 0.0, -t71 * t128 * t115 / 120.0 + 7.0 / 300.0 * t71 * t134 * t166 - t71 * t78 * t179 / 40.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t184 + 2.0 * t119;
        v2rhosigma[ip] += tv2rhosigma0;
        let t187 = t115 * t115;
        let t191 = t40 * t27;
        let t198 = t38 * t191 * t47 / 144.0 + t53 * sigma[ip] * t56 / 96.0;
        let t203 = piecewise3::<f64>(t2, 0.0, 7.0 / 300.0 * t71 * t134 * t187 - t71 * t78 * t198 / 40.0);
        let tv2sigma20 = 2.0 * rho[ip] * t203;
        v2sigma2[ip] += tv2sigma20;
        let t210 = t123 * t77;
        let t214 = t66 * t133;
        let t221 = t60 * t60;
        let t223 = 1.0 / t76 / t221;
        let t224 = t18 * t223;
        let t225 = t135 * t96;
        let t229 = t96 * t155;
        let t234 = 1.0 / t31 / t45;
        let t239 = 1.0 / t18 / t55;
        let t244 = 1.0 / t55 / t79;
        let t248 = -154.0 / 81.0 * t26 * t29 * t234 - 209.0 / 243.0 * t41 * t43 * t239 - 5.0 / 4.0 * t53 * t54 * t244;
        let t253 = piecewise3::<f64>(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t33 * t61 + t71 * t210 * t96 / 60.0 + 7.0 / 300.0 * t71 * t214 * t135 - t71 * t128 * t155 / 40.0 - 203.0 / 4500.0 * t71 * t224 * t225 + 7.0 / 100.0 * t71 * t134 * t229 - t71 * t78 * t248 / 40.0);
        let tv3rho30 = 2.0 * rho[ip] * t253 + 6.0 * t160;
        v3rho3[ip] += tv3rho30;
        let t266 = t115 * t135;
        let t270 = t179 * t96;
        let t274 = t115 * t155;
        let t287 = 11.0 / 27.0 * t21 * t104 * t140 + 19.0 / 81.0 * t41 * t108 * t146 + 3.0 / 8.0 * t53 * t42 * t151;
        let t292 = piecewise3::<f64>(t2, 0.0, t71 * t210 * t115 / 180.0 + 7.0 / 450.0 * t71 * t214 * t166 - t71 * t128 * t179 / 60.0 - 203.0 / 4500.0 * t71 * t224 * t266 + 7.0 / 150.0 * t71 * t134 * t270 + 7.0 / 300.0 * t71 * t134 * t274 - t71 * t78 * t287 / 40.0);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t292 + 4.0 * t184;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t298 = t187 * t96;
        let t302 = t115 * t179;
        let t309 = t198 * t96;
        let t319 = -t38 * t191 * t87 / 27.0 - t53 * sigma[ip] * t92 / 12.0;
        let t324 = piecewise3::<f64>(t2, 0.0, 7.0 / 900.0 * t71 * t214 * t187 - 203.0 / 4500.0 * t71 * t224 * t298 + 7.0 / 150.0 * t71 * t134 * t302 - t71 * t128 * t198 / 120.0 + 7.0 / 300.0 * t71 * t134 * t309 - t71 * t78 * t319 / 40.0);
        let tv3rhosigma20 = 2.0 * rho[ip] * t324 + 2.0 * t203;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t327 = t187 * t115;
        let t331 = t115 * t198;
        let t338 = t3 / t4 / t51 * t17;
        let t340 = 1.0 / t31 / t144;
        let t346 = piecewise3::<f64>(t2, 0.0, -203.0 / 4500.0 * t71 * t224 * t327 + 7.0 / 100.0 * t71 * t134 * t331 - t338 * t340 * t77 * param_cc / 3840.0);
        let tv3sigma30 = 2.0 * rho[ip] * t346;
        v3sigma3[ip] += tv3sigma30;
        let t353 = t33 * t77;
        let t357 = t123 * t133;
        let t364 = t66 * t223;
        let t377 = t18 / t76 / t221 / t60;
        let t378 = t135 * t135;
        let t386 = t155 * t155;
        let t413 = 10.0 / 27.0 * t6 * t17 * t81 * t61 - t71 * t353 * t96 / 27.0 - 7.0 / 225.0 * t71 * t357 * t135 + t71 * t210 * t155 / 30.0 - 203.0 / 3375.0 * t71 * t364 * t225 + 7.0 / 75.0 * t71 * t214 * t229 - t71 * t128 * t248 / 30.0 + 2233.0 / 16875.0 * t71 * t377 * t378 - 203.0 / 750.0 * t71 * t224 * t135 * t155 + 7.0 / 100.0 * t71 * t134 * t386 + 7.0 / 75.0 * t71 * t134 * t96 * t248 - t71 * t78 * (2618.0 / 243.0 * t26 * t29 / t31 / t85 + 5225.0 / 729.0 * t41 * t43 / t18 / t91 + 55.0 / 4.0 * t53 * t54 / t55 / t44) / 40.0;
        let t414 = piecewise3::<f64>(t2, 0.0, t413);
        let tv4rho40 = 2.0 * rho[ip] * t414 + 8.0 * t253;
        v4rho4[ip] += tv4rho40;
        let t425 = t6 * t19;
        let t426 = t223 * t115;
        let t477 = -203.0 / 4500.0 * t71 * t364 * t266 + 2233.0 / 16875.0 * t71 * t377 * t115 * t225 - 203.0 / 1500.0 * t425 * t426 * t229 + t71 * t210 * t179 / 60.0 - t71 * t128 * t287 / 40.0 - t71 * t78 * (-154.0 / 81.0 * t21 * t104 * t234 - 418.0 / 243.0 * t41 * t108 * t239 - 15.0 / 4.0 * t53 * t42 * t244) / 40.0 - 203.0 / 1500.0 * t71 * t224 * t179 * t135 + 7.0 / 100.0 * t71 * t134 * t287 * t96 + 7.0 / 100.0 * t71 * t134 * t179 * t155 + 7.0 / 300.0 * t71 * t134 * t115 * t248 - t71 * t353 * t115 / 108.0 - 7.0 / 450.0 * t71 * t357 * t166 + 7.0 / 150.0 * t71 * t214 * t270 + 7.0 / 300.0 * t71 * t214 * t274;
        let t478 = piecewise3::<f64>(t2, 0.0, t477);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t478 + 6.0 * t292;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t502 = t179 * t179;
        let t541 = -7.0 / 1350.0 * t71 * t357 * t187 - 203.0 / 6750.0 * t71 * t364 * t298 + 7.0 / 225.0 * t71 * t214 * t302 + 2233.0 / 16875.0 * t71 * t377 * t187 * t135 - 203.0 / 1125.0 * t425 * t426 * t270 - 203.0 / 4500.0 * t71 * t224 * t187 * t155 + 7.0 / 150.0 * t71 * t134 * t502 + 7.0 / 150.0 * t71 * t134 * t115 * t287 + t71 * t210 * t198 / 180.0 + 7.0 / 450.0 * t71 * t214 * t309 - t71 * t128 * t319 / 60.0 - 203.0 / 4500.0 * t71 * t224 * t198 * t135 + 7.0 / 150.0 * t71 * t134 * t319 * t96 + 7.0 / 300.0 * t71 * t134 * t198 * t155 - t71 * t78 * (19.0 / 81.0 * t38 * t191 * t146 + 3.0 / 4.0 * t53 * sigma[ip] * t151) / 40.0;
        let t542 = piecewise3::<f64>(t2, 0.0, t541);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t542 + 4.0 * t324;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t576 = t340 * t133;
        let t582 = piecewise3::<f64>(t2, 0.0, -203.0 / 13500.0 * t71 * t364 * t327 + 2233.0 / 16875.0 * t71 * t377 * t327 * t96 - 203.0 / 1500.0 * t71 * t224 * t187 * t179 + 7.0 / 300.0 * t71 * t214 * t331 - 203.0 / 1500.0 * t425 * t426 * t309 + 7.0 / 100.0 * t71 * t134 * t179 * t198 + 7.0 / 100.0 * t71 * t134 * t115 * t319 + 23.0 / 11520.0 * t338 / t31 / t55 * t77 * param_cc + 7.0 / 28800.0 * t338 * t576 * param_cc * t96);
        let tv4rhosigma30 = 2.0 * rho[ip] * t582 + 2.0 * t346;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t585 = t187 * t187;
        let t593 = t198 * t198;
        let t602 = piecewise3::<f64>(t2, 0.0, 2233.0 / 16875.0 * t71 * t377 * t585 - 203.0 / 750.0 * t71 * t224 * t187 * t198 + 7.0 / 100.0 * t71 * t134 * t593 + 7.0 / 7200.0 * t338 * t576 * t115 * param_cc);
        let tv4sigma40 = 2.0 * rho[ip] * t602;
        v4sigma4[ip] += tv4sigma40;
    }
}
