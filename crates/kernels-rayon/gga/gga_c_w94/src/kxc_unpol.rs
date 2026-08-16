//! GGA_C_W94 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_w94.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_w94_kxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = f64::sqrt(sigma[ip]);
        let t2 = t1 * sigma[ip];
        let t3 = rho[ip] * rho[ip];
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = pow_1_3(rho[ip]);
        let t9 = 1.0 / t7 / rho[ip];
        let t10 = t1 * t9;
        let t11 = f64::powf(t10, 1.0 / 16.0);
        let t12 = t11 * t11;
        let t13 = t12 * t11;
        let t16 = t3 * rho[ip];
        let t17 = 1.0 / t16;
        let t20 = M_CBRT3;
        let t22 = pow_1_3(1.0 / M_PI);
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
    }
}
