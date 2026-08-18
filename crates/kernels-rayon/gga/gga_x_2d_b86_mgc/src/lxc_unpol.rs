//! GGA_X_2D_B86_MGC lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86_mgc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_b86_mgc_lxc_unpol(
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
        let t19 = t17 * t18;
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t23 = sigma[ip] * t22;
        let t25 = 1.0 + 0.016646 * t23;
        let t26 = pow_1_4(t25);
        let t27 = t26 * t26;
        let t28 = t27 * t26;
        let t29 = 1.0 / t28;
        let t32 = 1.0 + 0.004409422067590198 * t23 * t29;
        let t36 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t32);
        let tzk0 = 2.0 * t36;
        zk[ip] += tzk0;
        let t38 = t17 / t18;
        let t42 = t20 * t20;
        let t43 = 1.0 / t42;
        let t47 = sigma[ip] * sigma[ip];
        let t48 = t42 * t21;
        let t49 = 1.0 / t48;
        let t52 = 1.0 / t28 / t25;
        let t55 = -0.013228266202770593 * sigma[ip] * t43 * t29 + 0.00016514828940848947 * t47 * t49 * t52;
        let t60 = piecewise3(t2, 0.0, -t16 * t38 * t32 / 3.0 - 2.0 / 3.0 * t16 * t19 * t55);
        let tvrho0 = 2.0 * rho[ip] * t60 + 2.0 * t36;
        vrho[ip] += tvrho0;
        let t65 = t42 * t20;
        let t66 = 1.0 / t65;
        let t67 = sigma[ip] * t66;
        let t70 = 0.004409422067590198 * t22 * t29 - 5.504942980282982e-05 * t67 * t52;
        let t74 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t70);
        let tvsigma0 = 2.0 * rho[ip] * t74;
        vsigma[ip] += tvsigma0;
        let t79 = t17 / t18 / rho[ip];
        let t86 = t42 * rho[ip];
        let t87 = 1.0 / t86;
        let t91 = t42 * t42;
        let t92 = 1.0 / t91;
        let t96 = t47 * sigma[ip];
        let t98 = 1.0 / t91 / t21;
        let t100 = t25 * t25;
        let t102 = 1.0 / t28 / t100;
        let t105 = 0.05291306481108237 * sigma[ip] * t87 * t29 - 0.0016514828940848946 * t47 * t92 * t52 + 1.4432556733842006e-05 * t96 * t98 * t102;
        let t110 = piecewise3(t2, 0.0, t16 * t79 * t32 / 6.0 - 2.0 / 3.0 * t16 * t38 * t55 - 2.0 / 3.0 * t16 * t19 * t105);
        let tv2rho20 = 2.0 * rho[ip] * t110 + 4.0 * t60;
        v2rho2[ip] += tv2rho20;
        let t118 = t49 * t52;
        let t122 = 1.0 / t91 / t20;
        let t123 = t47 * t122;
        let t126 = -0.013228266202770593 * t43 * t29 + 0.0004954448682254683 * t118 * sigma[ip] - 4.810852244614002e-06 * t123 * t102;
        let t131 = piecewise3(t2, 0.0, -t16 * t38 * t70 / 3.0 - 2.0 / 3.0 * t16 * t19 * t126);
        let tv2rhosigma0 = 2.0 * rho[ip] * t131 + 2.0 * t74;
        v2rhosigma[ip] += tv2rhosigma0;
        let t137 = 1.0 / t91 / rho[ip];
        let t141 = -0.00011009885960565965 * t66 * t52 + 1.6036174148713342e-06 * sigma[ip] * t137 * t102;
        let t145 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t141);
        let tv2sigma20 = 2.0 * rho[ip] * t145;
        v2sigma2[ip] += tv2sigma20;
        let t150 = t17 / t18 / t20;
        let t165 = 1.0 / t91 / t42;
        let t169 = t47 * t47;
        let t171 = 1.0 / t91 / t48;
        let t175 = 1.0 / t28 / t100 / t25;
        let t178 = -0.26456532405541183 * t67 * t29 + 0.01519364262558103 * t47 * t137 * t52 - 0.0003030836914106821 * t96 * t165 * t102 + 1.9820157999801557e-06 * t169 * t171 * t175;
        let t183 = piecewise3(t2, 0.0, -t16 * t150 * t32 / 4.0 + t16 * t79 * t55 / 2.0 - t16 * t38 * t105 - 2.0 / 3.0 * t16 * t19 * t178);
        let tv3rho30 = 2.0 * rho[ip] * t183 + 6.0 * t110;
        v3rho3[ip] += tv3rho30;
        let t195 = t92 * t52;
        let t198 = t98 * t102;
        let t202 = 1.0 / t91 / t65;
        let t206 = 0.05291306481108237 * t87 * t29 - 0.003963558945803747 * t195 * sigma[ip] + 9.140619264766605e-05 * t198 * t47 - 6.606719333267186e-07 * t96 * t202 * t175;
        let t211 = piecewise3(t2, 0.0, t16 * t79 * t70 / 6.0 - 2.0 / 3.0 * t16 * t38 * t126 - 2.0 / 3.0 * t16 * t19 * t206);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t211 + 4.0 * t131;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t218 = t122 * t102;
        let t222 = 1.0 / t91 / t86;
        let t226 = 0.0006605931576339579 * t118 - 2.405426122307001e-05 * t218 * sigma[ip] + 2.2022397777557288e-07 * t47 * t222 * t175;
        let t231 = piecewise3(t2, 0.0, -t16 * t38 * t141 / 3.0 - 2.0 / 3.0 * t16 * t19 * t226);
        let tv3rhosigma20 = 2.0 * rho[ip] * t231 + 2.0 * t145;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t239 = 4.810852244614002e-06 * t137 * t102 - 7.340799259185763e-08 * sigma[ip] * t165 * t175;
        let t243 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t239);
        let tv3sigma30 = 2.0 * rho[ip] * t243;
        v3sigma3[ip] += tv3sigma30;
        let t267 = t91 * t91;
        let t268 = 1.0 / t267;
        let t276 = t100 * t100;
        let t278 = 1.0 / t28 / t276;
        let t286 = piecewise3(t2, 0.0, 5.0 / 8.0 * t16 * t17 / t18 / t21 * t32 - t16 * t150 * t55 + t16 * t79 * t105 - 4.0 / 3.0 * t16 * t38 * t178 - 2.0 / 3.0 * t16 * t19 * (1.587391944332471 * sigma[ip] * t49 * t29 - 0.14665168099473863 * t123 * t52 + 0.0049647995164416505 * t96 * t222 * t102 - 7.135256879928562e-05 * t169 * t268 * t175 + 3.7116714382278384e-07 * t169 * sigma[ip] / t267 / t21 * t278));
        let tv4rho40 = 2.0 * rho[ip] * t286 + 8.0 * t183;
        v4rho4[ip] += tv4rho40;
        let t319 = piecewise3(t2, 0.0, -t16 * t150 * t70 / 4.0 + t16 * t79 * t126 / 2.0 - t16 * t38 * t206 - 2.0 / 3.0 * t16 * t19 * (-0.26456532405541183 * t66 * t29 + 0.03369025103933185 * t137 * t52 * sigma[ip] - 0.0013518494807365346 * t165 * t102 * t47 + 2.1802173799781715e-05 * t171 * t175 * t96 - 1.2372238127426128e-07 * t169 / t267 / t20 * t278));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t319 + 6.0 * t211;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t345 = piecewise3(t2, 0.0, t16 * t79 * t141 / 6.0 - 2.0 / 3.0 * t16 * t38 * t226 - 2.0 / 3.0 * t16 * t19 * (-0.004624152103437705 * t195 + 0.00029827283916606816 * t198 * sigma[ip] - 6.1662713777160406e-06 * t202 * t175 * t47 + 4.124079375808709e-08 * t96 / t267 / rho[ip] * t278));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t345 + 4.0 * t231;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t363 = piecewise3(t2, 0.0, -t16 * t38 * t239 / 3.0 - 2.0 / 3.0 * t16 * t19 * (-4.329767020152602e-05 * t218 + 1.5415678444290101e-06 * t222 * t175 * sigma[ip] - 1.3746931252695698e-08 * t47 * t268 * t278));
        let tv4rhosigma30 = 2.0 * rho[ip] * t363 + 2.0 * t243;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t375 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * (-2.936319703674305e-07 * t165 * t175 + 4.582310417565232e-09 * sigma[ip] * t171 * t278));
        let tv4sigma40 = 2.0 * rho[ip] * t375;
        v4sigma4[ip] += tv4sigma40;
    }
}
