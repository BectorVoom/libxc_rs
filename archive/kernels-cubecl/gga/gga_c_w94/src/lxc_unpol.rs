//! GGA_C_W94 lxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_w94.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_w94_lxc_unpol(
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
        let t1 = f64::sqrt(sigma[ip]);
        let t2 = t1 * sigma[ip];
        let t3 = rho[ip] * rho[ip];
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = pow_1_3::<f64>(rho[ip]);
        let t9 = 1.0 / t7 / rho[ip];
        let t10 = t1 * t9;
        let t11 = f64::powf(t10, 1.0 / 16.0);
        let t12 = t11 * t11;
        let t13 = t12 * t11;
        let t16 = t3 * rho[ip];
        let t17 = 1.0 / t16;
        let t20 = M_CBRT3;
        let t22 = pow_1_3::<f64>(1.0 / M_PI);
        let t23 = t20 * t22;
        let t24 = M_CBRT4;
        let t25 = t24 * t24;
        let t30 = 0.118e2 + 0.15067e0 * t13 * t2 * t5 + 0.1102e-1 * sigma[ip] * t17 + t23 * t25 / t7 / 4.0;
        let tzk0 = -1.0 / t30;
        zk[ip] += tzk0;
        let t32 = t30 * t30;
        let t33 = 1.0 / t32;
        let t34 = rho[ip] * t33;
        let t35 = t7 * t7;
        let t37 = 1.0 / t35 / t3;
        let t39 = t13 * sigma[ip] * t37;
        let t40 = t39 * t1;
        let t42 = 1.0 / t7 / t3;
        let t50 = -0.6403475e0 * t40 * t42 - 0.3306e-1 * sigma[ip] * t5 - t23 * t25 * t9 / 12.0;
        let tvrho0 = t34 * t50 + tzk0;
        vrho[ip] += tvrho0;
        let t52 = 1.0 / t1;
        let t53 = t39 * t52;
        let t57 = 0.2401303125e0 * t53 * t9 + 0.1102e-1 * t17;
        let tvsigma0 = t34 * t57;
        vsigma[ip] += tvsigma0;
        let t61 = 1.0 / t32 / t30;
        let t62 = rho[ip] * t61;
        let t63 = t50 * t50;
        let t66 = t13 * t10;
        let t67 = t66 * sigma[ip];
        let t69 = 1.0 / t35 / t4;
        let t73 = 1.0 / t7 / t16;
        let t76 = t4 * rho[ip];
        let t77 = 1.0 / t76;
        let t83 = 0.18676802083333333333e1 * t67 * t69 + 0.14941441666666666667e1 * t40 * t73 + 0.13224e0 * sigma[ip] * t77 + t23 * t25 * t42 / 9.0;
        let tv2rho20 = 2.0 * t33 * t50 + t34 * t83 - 2.0 * t62 * t63;
        v2rho2[ip] += tv2rho20;
        let t86 = t57 * t50;
        let t90 = 1.0 / t35 / t16;
        let t96 = -0.700380078125e0 * t66 * t90 - 0.32017375e0 * t53 * t42 - 0.3306e-1 * t5;
        let tv2rhosigma0 = t33 * t57 + t34 * t96 - 2.0 * t62 * t86;
        v2rhosigma[ip] += tv2rhosigma0;
        let t98 = t57 * t57;
        let t101 = 1.0 / sigma[ip];
        let t102 = t66 * t101;
        let t105 = 1.0 / t2;
        let t106 = t39 * t105;
        let t109 = 0.262642529296875e0 * t102 * t37 - 0.12006515625e0 * t106 * t9;
        let tv2sigma20 = t34 * t109 - 2.0 * t62 * t98;
        v2sigma2[ip] += tv2sigma20;
        let t115 = t32 * t32;
        let t116 = 1.0 / t115;
        let t117 = rho[ip] * t116;
        let t118 = t63 * t50;
        let t124 = t13 * t2;
        let t125 = t4 * t16;
        let t126 = 1.0 / t125;
        let t130 = 1.0 / t35 / t76;
        let t134 = 1.0 / t7 / t4;
        let t137 = t4 * t3;
        let t138 = 1.0 / t137;
        let t144 = -0.29571603298611111111e1 * t124 * t126 - 0.13073761458333333333e2 * t67 * t130 - 0.49804805555555555557e1 * t40 * t134 - 0.6612e0 * sigma[ip] * t138 - 7.0 / 27.0 * t23 * t25 * t73;
        let tv3rho30 = -6.0 * t62 * t50 * t83 + 6.0 * t117 * t118 + t34 * t144 + 3.0 * t33 * t83 - 6.0 * t61 * t63;
        v3rho3[ip] += tv3rho30;
        let t146 = t61 * t57;
        let t168 = 0.11089351236979166667e1 * t13 * t138 * t1 + 0.3501900390625e1 * t66 * t69 + 0.74707208333333333333e0 * t53 * t73 + 0.13224e0 * t77;
        let tv3rho2sigma0 = 6.0 * t117 * t57 * t63 - 4.0 * t62 * t96 * t50 - 2.0 * t62 * t57 * t83 - 4.0 * t146 * t50 + t34 * t168 + 2.0 * t33 * t96;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t182 = t13 * t52;
        let t189 = -0.41585067138671875e0 * t182 * t77 - 0.3501900390625e0 * t102 * t90 + 0.160086875e0 * t106 * t42;
        let tv3rhosigma20 = -2.0 * t62 * t109 * t50 + 6.0 * t117 * t98 * t50 - 4.0 * t62 * t57 * t96 + t33 * t109 + t34 * t189 - 2.0 * t61 * t98;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t191 = t98 * t57;
        let t194 = t57 * t109;
        let t200 = sigma[ip] * sigma[ip];
        let t201 = 1.0 / t200;
        let t202 = t66 * t201;
        let t206 = 1.0 / t1 / t200;
        let t207 = t39 * t206;
        let t210 = 0.15594400177001953125e0 * t13 * t105 * t5 - 0.3939637939453125e0 * t202 * t37 + 0.180097734375e0 * t207 * t9;
        let tv3sigma30 = 6.0 * t117 * t191 - 6.0 * t62 * t194 + t34 * t210;
        v3sigma3[ip] += tv3sigma30;
        let t221 = rho[ip] / t115 / t30;
        let t222 = t63 * t63;
        let t228 = t83 * t83;
        let t234 = t12 * t12;
        let t236 = t234 * t234;
        let t238 = 1.0 / t236 / t234 / t11;
        let t240 = t4 * t4;
        let t254 = 1.0 / t7 / t76;
        let tv4rho40 = 24.0 * t116 * t118 - 24.0 * t61 * t50 * t83 + 4.0 * t33 * t144 - 24.0 * t221 * t222 + 36.0 * t117 * t63 * t83 - 6.0 * t62 * t228 - 8.0 * t62 * t50 * t144 + t34 * (0.73929008246527777778e0 * t238 * t200 / t7 / t240 / rho[ip] + 0.41400244618055555555e2 * t124 / t240 + 0.88611049884259259258e2 * t67 / t35 / t137 + 0.21582082407407407408e2 * t40 * t254 + 0.39672e1 * sigma[ip] * t126 + 70.0 / 81.0 * t23 * t25 * t134);
        v4rho4[ip] += tv4rho40;
        let tv4rho3sigma0 = 18.0 * t116 * t57 * t63 - 12.0 * t61 * t96 * t50 - 6.0 * t146 * t83 + 3.0 * t33 * t168 - 24.0 * t221 * t57 * t118 + 18.0 * t117 * t96 * t63 + 18.0 * t117 * t86 * t83 - 6.0 * t62 * t168 * t50 - 6.0 * t62 * t96 * t83 - 2.0 * t62 * t57 * t144 + t34 * (-0.27723378092447916668e0 * t238 / t7 / t240 * sigma[ip] - 0.12198286360677083334e2 * t13 * t126 * t1 - 0.18521162065972222222e2 * t66 * t130 - 0.24902402777777777778e1 * t53 * t134 - 0.6612e0 * t138);
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t321 = t96 * t96;
        let tv4rho2sigma20 = 12.0 * t116 * t98 * t50 - 8.0 * t146 * t96 - 24.0 * t221 * t98 * t63 + 24.0 * t117 * t86 * t96 + 6.0 * t117 * t98 * t83 - 4.0 * t62 * t321 - 4.0 * t62 * t57 * t168 - 4.0 * t61 * t109 * t50 + 2.0 * t33 * t189 + 6.0 * t117 * t109 * t63 - 4.0 * t62 * t189 * t50 - 2.0 * t62 * t109 * t83 + t34 * (0.1039626678466796875e0 * t238 / t7 / t125 + 0.26337209187825520833e1 * t182 * t138 + 0.81711009114583333337e0 * t102 * t69 - 0.37353604166666666667e0 * t106 * t73);
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let tv4rhosigma30 = 6.0 * t116 * t191 - 24.0 * t221 * t191 * t50 + 18.0 * t117 * t98 * t96 - 6.0 * t146 * t109 + 18.0 * t117 * t194 * t50 - 6.0 * t62 * t96 * t109 - 6.0 * t62 * t57 * t189 + t33 * t210 - 2.0 * t62 * t210 * t50 + t34 * (-0.38986000442504882812e-1 * t238 * t101 / t7 / t137 + 0.52528505859375e0 * t202 * t90 - 0.2401303125e0 * t207 * t42);
        v4rhosigma3[ip] += tv4rhosigma30;
        let t387 = t98 * t98;
        let t393 = t109 * t109;
        let t405 = t200 * sigma[ip];
        let tv4sigma40 = -24.0 * t221 * t387 + 36.0 * t117 * t98 * t109 - 6.0 * t62 * t393 - 8.0 * t62 * t57 * t210 + t34 * (0.14619750165939331055e-1 * t238 * t201 * t254 - 0.46783200531005859376e0 * t13 * t206 * t5 + 0.98490948486328125e0 * t66 / t405 * t37 - 0.4502443359375e0 * t39 / t1 / t405 * t9);
        v4sigma4[ip] += tv4sigma40;
    }
}
