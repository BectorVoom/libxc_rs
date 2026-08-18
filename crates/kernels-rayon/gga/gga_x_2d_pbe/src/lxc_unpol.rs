//! GGA_X_2D_PBE lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_pbe_lxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = f64::sqrt(rho[ip]);
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t25 = 0.4604 + 0.014106971928508582 * sigma[ip] / t21;
        let t28 = 1.4604 - 0.21196816 / t25;
        let t32 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t17 * t18 * t28);
        let tzk0 = 2.0 * t32;
        zk[ip] += tzk0;
        let t38 = t15 * t17;
        let t40 = 1.0 / t18 / t21;
        let t41 = t25 * t25;
        let t42 = 1.0 / t41;
        let t43 = t40 * t42;
        let t48 = piecewise3(t2, 0.0, -t16 * t17 / t18 * t28 / 3.0 + 0.0033741119762638215 * t38 * t43 * sigma[ip]);
        let tvrho0 = 2.0 * rho[ip] * t48 + 2.0 * t32;
        vrho[ip] += tvrho0;
        let t52 = 1.0 / t18 / t20;
        let t56 = piecewise3(t2, 0.0, -0.0011247039920879406 * t38 * t52 * t42);
        let tvsigma0 = 2.0 * rho[ip] * t56;
        vsigma[ip] += tvsigma0;
        let t65 = t20 * t20;
        let t68 = 1.0 / t18 / t65 * t42;
        let t72 = t65 * t21;
        let t76 = 1.0 / t41 / t25;
        let t77 = 1.0 / t18 / t72 * t76;
        let t78 = sigma[ip] * sigma[ip];
        let t83 = piecewise3(t2, 0.0, t16 * t17 / t18 / rho[ip] * t28 / 6.0 - 0.010122335928791465 * t38 * t68 * sigma[ip] + 0.0002855910175967901 * t38 * t77 * t78);
        let tv2rho20 = 2.0 * rho[ip] * t83 + 4.0 * t48;
        v2rho2[ip] += tv2rho20;
        let t88 = t65 * t20;
        let t90 = 1.0 / t18 / t88;
        let t91 = t90 * t76;
        let t96 = piecewise3(t2, 0.0, 0.002811759980219851 * t38 * t43 - 9.51970058655967e-05 * t38 * t91 * sigma[ip]);
        let tv2rhosigma0 = 2.0 * rho[ip] * t96 + 2.0 * t56;
        v2rhosigma[ip] += tv2rhosigma0;
        let t99 = t65 * rho[ip];
        let t101 = 1.0 / t18 / t99;
        let t105 = piecewise3(t2, 0.0, 3.173233528853223e-05 * t38 * t101 * t76);
        let tv2sigma20 = 2.0 * rho[ip] * t105;
        v2sigma2[ip] += tv2sigma20;
        let t112 = t101 * t42;
        let t116 = t65 * t65;
        let t118 = 1.0 / t18 / t116;
        let t119 = t118 * t76;
        let t125 = 1.0 / t18 / t116 / t21;
        let t126 = t41 * t41;
        let t127 = 1.0 / t126;
        let t128 = t125 * t127;
        let t129 = t78 * sigma[ip];
        let t134 = piecewise3(t2, 0.0, -t16 * t17 * t52 * t28 / 4.0 + 0.04470698368549563 * t38 * t112 * sigma[ip] - 0.0029987056847662957 * t38 * t119 * t78 + 3.6259420214449066e-05 * t38 * t128 * t129);
        let tv3rho30 = 2.0 * rho[ip] * t134 + 6.0 * t83;
        v3rho3[ip] += tv3rho30;
        let t146 = 1.0 / t18 / t116 / t20 * t127;
        let t151 = piecewise3(t2, 0.0, -0.00984115993076948 * t38 * t68 + 0.0008567730527903702 * t38 * t77 * sigma[ip] - 1.2086473404816354e-05 * t38 * t146 * t78);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t151 + 4.0 * t96;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t158 = 1.0 / t18 / t116 / rho[ip];
        let t159 = t158 * t127;
        let t164 = piecewise3(t2, 0.0, -0.00017452784408692726 * t38 * t91 + 4.028824468272118e-06 * t38 * t159 * sigma[ip]);
        let tv3rhosigma20 = 2.0 * rho[ip] * t164 + 2.0 * t105;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t170 = piecewise3(t2, 0.0, -1.3429414894240394e-06 * t38 * t118 * t127);
        let tv3sigma30 = 2.0 * rho[ip] * t170;
        v3sigma3[ip] += tv3sigma30;
        let t187 = 1.0 / t18 / t116 / t65;
        let t196 = 1.0 / t126 / t25;
        let t198 = t78 * t78;
        let t203 = piecewise3(t2, 0.0, 5.0 / 8.0 * t16 * t17 * t40 * t28 - 0.24462311827912706 * t38 * t90 * t42 * sigma[ip] + 0.02927307930367098 * t38 * t158 * t76 * t78 - 0.0007977072447178794 * t38 * t187 * t127 * t129 + 6.1381274773107546e-06 * t38 / t18 / t116 / t72 * t196 * t198);
        let tv4rho40 = 2.0 * rho[ip] * t203 + 8.0 * t134;
        v4rho4[ip] += tv4rho40;
        let t223 = piecewise3(t2, 0.0, 0.044285219688462656 * t38 * t112 - 0.0072587716972517475 * t38 * t119 * sigma[ip] + 0.0002356862313939189 * t38 * t128 * t78 - 2.046042492436918e-06 * t38 / t18 / t116 / t88 * t196 * t129);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t223 + 6.0 * t151;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t240 = piecewise3(t2, 0.0, 0.0011344309865650272 * t38 * t77 - 6.0432367024081774e-05 * t38 * t146 * sigma[ip] + 6.820141641456394e-07 * t38 / t18 / t116 / t99 * t196 * t78);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t240 + 4.0 * t164;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t250 = piecewise3(t2, 0.0, 1.1415002660104335e-05 * t38 * t159 - 2.2733805471521313e-07 * t38 * t187 * t196 * sigma[ip]);
        let tv4rhosigma30 = 2.0 * rho[ip] * t250 + 2.0 * t170;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t256 = piecewise3(t2, 0.0, 7.577935157173772e-08 * t38 * t125 * t196);
        let tv4sigma40 = 2.0 * rho[ip] * t256;
        v4sigma4[ip] += tv4sigma40;
    }
}
