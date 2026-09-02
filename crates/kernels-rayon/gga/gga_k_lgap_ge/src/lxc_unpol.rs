//! GGA_K_LGAP_GE lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap_ge.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lgap_ge_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
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
        let t23 = t20 * t22;
        let t25 = M_CBRT6;
        let t26 = t25 * t25;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t31 = param_mu_0 * t26 / t29;
        let t32 = rmath::sqrt(sigma[ip]);
        let t33 = M_CBRT2;
        let t34 = t32 * t33;
        let t36 = 1.0 / t21 / rho[ip];
        let t41 = param_mu_1 * t25;
        let t42 = t29 * t29;
        let t43 = 1.0 / t42;
        let t44 = t41 * t43;
        let t45 = t33 * t33;
        let t46 = sigma[ip] * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t22 / t47;
        let t55 = param_mu_2 / t28;
        let t56 = t32 * sigma[ip];
        let t57 = t47 * t47;
        let t58 = 1.0 / t57;
        let t62 = 1.0 + t31 * t34 * t36 / 12.0 + t44 * t46 * t49 / 24.0 + t55 * t56 * t58 / 24.0;
        let t66 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t62);
        let tzk0 = 2.0 * t66;
        zk[ip] += tzk0;
        let t68 = t20 / t21;
        let t73 = 1.0 / t21 / t47;
        let t77 = t47 * rho[ip];
        let t79 = 1.0 / t22 / t77;
        let t83 = t57 * rho[ip];
        let t84 = 1.0 / t83;
        let t88 = -t31 * t34 * t73 / 9.0 - t44 * t46 * t79 / 9.0 - t55 * t56 * t84 / 6.0;
        let t93 = piecewise3(t2, 0.0, t7 * t68 * t62 / 10.0 + 3.0 / 20.0 * t7 * t23 * t88);
        let tvrho0 = 2.0 * rho[ip] * t93 + 2.0 * t66;
        vrho[ip] += tvrho0;
        let t96 = 1.0 / t32;
        let t97 = t96 * t33;
        let t101 = t43 * t45;
        let t108 = t31 * t97 * t36 / 24.0 + t41 * t101 * t49 / 24.0 + t55 * t32 * t58 / 16.0;
        let t112 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t108);
        let tvsigma0 = 2.0 * rho[ip] * t112;
        vsigma[ip] += tvsigma0;
        let t115 = t20 * t36;
        let t123 = 1.0 / t21 / t77;
        let t128 = 1.0 / t22 / t57;
        let t132 = t57 * t47;
        let t133 = 1.0 / t132;
        let t137 = 7.0 / 27.0 * t31 * t34 * t123 + 11.0 / 27.0 * t44 * t46 * t128 + 5.0 / 6.0 * t55 * t56 * t133;
        let t142 = piecewise3(t2, 0.0, -t7 * t115 * t62 / 30.0 + t7 * t68 * t88 / 5.0 + 3.0 / 20.0 * t7 * t23 * t137);
        let tv2rho20 = 2.0 * rho[ip] * t142 + 4.0 * t93;
        v2rho2[ip] += tv2rho20;
        let t157 = -t31 * t97 * t73 / 18.0 - t41 * t101 * t79 / 9.0 - t55 * t32 * t84 / 4.0;
        let t162 = piecewise3(t2, 0.0, t7 * t68 * t108 / 10.0 + 3.0 / 20.0 * t7 * t23 * t157);
        let tv2rhosigma0 = 2.0 * rho[ip] * t162 + 2.0 * t112;
        v2rhosigma[ip] += tv2rhosigma0;
        let t165 = 1.0 / t56;
        let t166 = t165 * t33;
        let t173 = -t31 * t166 * t36 / 48.0 + t55 * t96 * t58 / 32.0;
        let t177 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t173);
        let tv2sigma20 = 2.0 * rho[ip] * t177;
        v2sigma2[ip] += tv2sigma20;
        let t180 = t20 * t73;
        let t191 = 1.0 / t21 / t57;
        let t196 = 1.0 / t22 / t83;
        let t201 = 1.0 / t57 / t77;
        let t205 = -70.0 / 81.0 * t31 * t34 * t191 - 154.0 / 81.0 * t44 * t46 * t196 - 5.0 * t55 * t56 * t201;
        let t210 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t180 * t62 - t7 * t115 * t88 / 10.0 + 3.0 / 10.0 * t7 * t68 * t137 + 3.0 / 20.0 * t7 * t23 * t205);
        let tv3rho30 = 2.0 * rho[ip] * t210 + 6.0 * t142;
        v3rho3[ip] += tv3rho30;
        let t229 = 7.0 / 54.0 * t31 * t97 * t123 + 11.0 / 27.0 * t41 * t101 * t128 + 5.0 / 4.0 * t55 * t32 * t133;
        let t234 = piecewise3(t2, 0.0, -t7 * t115 * t108 / 30.0 + t7 * t68 * t157 / 5.0 + 3.0 / 20.0 * t7 * t23 * t229);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t234 + 4.0 * t162;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t246 = t31 * t166 * t73 / 36.0 - t55 * t96 * t84 / 8.0;
        let t251 = piecewise3(t2, 0.0, t7 * t68 * t173 / 10.0 + 3.0 / 20.0 * t7 * t23 * t246);
        let tv3rhosigma20 = 2.0 * rho[ip] * t251 + 2.0 * t177;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t254 = sigma[ip] * sigma[ip];
        let t256 = 1.0 / t32 / t254;
        let t257 = t256 * t33;
        let t264 = t31 * t257 * t36 / 32.0 - t55 * t165 * t58 / 64.0;
        let t268 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t264);
        let tv3sigma30 = 2.0 * rho[ip] * t268;
        v3sigma3[ip] += tv3sigma30;
        let t294 = t57 * t57;
        let t304 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 * t123 * t62 + 8.0 / 45.0 * t7 * t180 * t88 - t7 * t115 * t137 / 5.0 + 2.0 / 5.0 * t7 * t68 * t205 + 3.0 / 20.0 * t7 * t23 * (910.0 / 243.0 * t31 * t34 / t21 / t83 + 2618.0 / 243.0 * t44 * t46 / t22 / t132 + 35.0 * t55 * t56 / t294));
        let tv4rho40 = 2.0 * rho[ip] * t304 + 8.0 * t210;
        v4rho4[ip] += tv4rho40;
        let t331 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t180 * t108 - t7 * t115 * t157 / 10.0 + 3.0 / 10.0 * t7 * t68 * t229 + 3.0 / 20.0 * t7 * t23 * (-35.0 / 81.0 * t31 * t97 * t191 - 154.0 / 81.0 * t41 * t101 * t196 - 15.0 / 2.0 * t55 * t32 * t201));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t331 + 6.0 * t234;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t352 = piecewise3(t2, 0.0, -t7 * t115 * t173 / 30.0 + t7 * t68 * t246 / 5.0 + 3.0 / 20.0 * t7 * t23 * (-7.0 / 108.0 * t31 * t166 * t123 + 5.0 / 8.0 * t55 * t96 * t133));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t352 + 4.0 * t251;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t369 = piecewise3(t2, 0.0, t7 * t68 * t264 / 10.0 + 3.0 / 20.0 * t7 * t23 * (-t31 * t257 * t73 / 24.0 + t55 * t165 * t84 / 16.0));
        let tv4rhosigma30 = 2.0 * rho[ip] * t369 + 2.0 * t268;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t386 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * (-5.0 / 64.0 * t31 / t32 / t254 / sigma[ip] * t33 * t36 + 3.0 / 128.0 * t55 * t256 * t58));
        let tv4sigma40 = 2.0 * rho[ip] * t386;
        v4sigma4[ip] += tv4sigma40;
    }
}
