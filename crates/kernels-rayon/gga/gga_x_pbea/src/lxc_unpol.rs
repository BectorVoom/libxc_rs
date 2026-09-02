//! GGA_X_PBEA lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbea_lxc_unpol(
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
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t20 = M_CBRT2;
        let t21 = t20 * t20;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t29 = 1.0 + 0.008639940809536326 * sigma[ip] * t21 * t26;
        let t30 = rmath::pow(t29, -0.52);
        let t32 = 1.804 - 0.804 * t30;
        let t36 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t32);
        let tzk0 = 2.0 * t36;
        zk[ip] += tzk0;
        let t42 = t3 * t17;
        let t43 = t23 * rho[ip];
        let t45 = 1.0 / t18 / t43;
        let t47 = rmath::pow(t29, -1.52);
        let t49 = t47 * sigma[ip] * t21;
        let t53 = piecewise3(t2, 0.0, -t6 * t17 / t24 * t32 / 8.0 + 0.00246634334405953 * t42 * t45 * t49);
        let tvrho0 = 2.0 * rho[ip] * t53 + 2.0 * t36;
        vrho[ip] += tvrho0;
        let t62 = piecewise3(t2, 0.0, -0.0009248787540223239 * t42 / t18 / t23 * t47 * t21);
        let tvsigma0 = 2.0 * rho[ip] * t62;
        vsigma[ip] += tvsigma0;
        let t71 = t23 * t23;
        let t73 = 1.0 / t18 / t71;
        let t77 = t71 * t43;
        let t78 = 1.0 / t77;
        let t79 = t42 * t78;
        let t80 = rmath::pow(t29, -2.52);
        let t81 = sigma[ip] * sigma[ip];
        let t83 = t80 * t81 * t20;
        let t87 = piecewise3(t2, 0.0, t6 * t17 / t24 / rho[ip] * t32 / 12.0 - 0.007399030032178591 * t42 * t73 * t49 + 0.00017274545052360375 * t79 * t83);
        let tv2rho20 = 2.0 * rho[ip] * t87 + 4.0 * t53;
        v2rho2[ip] += tv2rho20;
        let t94 = t71 * t23;
        let t95 = 1.0 / t94;
        let t98 = t80 * t20 * sigma[ip];
        let t102 = piecewise3(t2, 0.0, 0.002158050426052089 * t42 * t45 * t47 * t21 - 6.47795439463514e-05 * t42 * t95 * t98);
        let tv2rhosigma0 = 2.0 * rho[ip] * t102 + 2.0 * t62;
        v2rhosigma[ip] += tv2rhosigma0;
        let t105 = t71 * rho[ip];
        let t111 = piecewise3(t2, 0.0, 2.429232897988178e-05 * t42 / t105 * t80 * t20);
        let tv2sigma20 = 2.0 * rho[ip] * t111;
        v2sigma2[ip] += tv2sigma20;
        let t119 = 1.0 / t18 / t105;
        let t123 = t71 * t71;
        let t125 = t42 / t123;
        let t128 = t123 * t23;
        let t131 = rmath::pow(t29, -3.52);
        let t132 = 1.0 / t24 / t128 * t131;
        let t133 = t81 * sigma[ip];
        let t138 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t17 * t26 * t32 + 0.031514387174094 * t42 * t119 * t49 - 0.0017274545052360377 * t125 * t83 + 2.0059340685089964e-05 * t42 * t132 * t133);
        let tv3rho30 = 2.0 * rho[ip] * t138 + 6.0 * t87;
        v3rho3[ip] += tv3rho30;
        let t148 = t123 * rho[ip];
        let t151 = 1.0 / t24 / t148 * t131;
        let t156 = piecewise3(t2, 0.0, -0.00719350142017363 * t42 * t73 * t47 * t21 + 0.0005398295328862617 * t79 * t98 - 7.522252756908737e-06 * t42 * t151 * t81);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t156 + 4.0 * t102;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t165 = 1.0 / t24 / t123 * t131;
        let t170 = piecewise3(t2, 0.0, -0.0001214616448994089 * t42 * t95 * t80 * t20 + 2.820844783840776e-06 * t42 * t165 * sigma[ip]);
        let tv3rhosigma20 = 2.0 * rho[ip] * t170 + 2.0 * t111;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t178 = piecewise3(t2, 0.0, -1.0578167939402912e-06 * t42 / t24 / t77 * t131);
        let tv3sigma30 = 2.0 * rho[ip] * t178;
        v3sigma3[ip] += tv3sigma30;
        let t196 = t123 * t43;
        let t207 = rmath::pow(t29, -4.52);
        let t208 = t81 * t81;
        let t214 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 / t24 / t43 * t32 - 0.16716327109736817 * t42 / t18 / t94 * t49 + 0.016026939020801014 * t42 / t148 * t83 - 0.00041455970749185926 * t42 / t24 / t196 * t131 * t133 + 1.626817432040544e-06 * t42 / t18 / t123 / t94 * t207 * t208 * t21);
        let tv4rho40 = 2.0 * rho[ip] * t214 + 8.0 * t138;
        v4rho4[ip] += tv4rho40;
        let t236 = piecewise3(t2, 0.0, 0.031171839487419063 * t42 * t119 * t47 * t21 - 0.004282647627564343 * t125 * t98 + 0.00013540054962435727 * t42 * t132 * t81 - 6.10056537015204e-07 * t42 / t18 / t123 / t105 * t207 * t133 * t21);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t236 + 6.0 * t156;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t256 = piecewise3(t2, 0.0, 0.0007287698693964534 * t42 * t78 * t80 * t20 - 3.855154537915727e-05 * t42 * t151 * sigma[ip] + 2.287712013807015e-07 * t42 / t18 / t123 / t71 * t207 * t81 * t21);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t256 + 4.0 * t170;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t269 = piecewise3(t2, 0.0, 8.109928753542231e-06 * t42 * t165 - 8.578920051776306e-08 * t42 / t18 / t196 * t207 * sigma[ip] * t21);
        let tv4rhosigma30 = 2.0 * rho[ip] * t269 + 2.0 * t178;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t278 = piecewise3(t2, 0.0, 3.217095019416115e-08 * t42 / t18 / t128 * t207 * t21);
        let tv4sigma40 = 2.0 * rho[ip] * t278;
        v4sigma4[ip] += tv4sigma40;
    }
}
