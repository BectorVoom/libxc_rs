//! GGA_X_2D_B86 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_b86_kxc_unpol(
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
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = rmath::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = rmath::sqrt(zeta_threshold);
        let t13 = rmath::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t17 = M_SQRT2;
        let t18 = 1.0 / t3 * t15 * t17;
        let t19 = rmath::sqrt(rho[ip]);
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t23 = sigma[ip] / t21;
        let t25 = 1.0 + 0.00421 * t23;
        let t28 = 1.0 + 0.000238 * t23;
        let t29 = 1.0 / t28;
        let t33 = piecewise3(t2, 0.0, -2.0 / 3.0 * t18 * t19 * t25 * t29);
        let tzk0 = 2.0 * t33;
        zk[ip] += tzk0;
        let t39 = t15 * t17;
        let t41 = 1.0 / t19 / t21;
        let t47 = t28 * t28;
        let t48 = 1.0 / t47;
        let t50 = t25 * t48 * sigma[ip];
        let t54 = piecewise3(t2, 0.0, -t18 / t19 * t25 * t29 / 3.0 + 0.004750476293472108 * t39 * t41 * sigma[ip] * t29 - 0.000268554241768732 * t39 * t41 * t50);
        let tvrho0 = 2.0 * rho[ip] * t54 + 2.0 * t33;
        vrho[ip] += tvrho0;
        let t58 = 1.0 / t19 / t20;
        let t62 = t58 * t25;
        let t67 = piecewise3(t2, 0.0, -0.001583492097824036 * t39 * t58 * t29 + 8.951808058957734e-05 * t39 * t62 * t48);
        let tvsigma0 = 2.0 * rho[ip] * t67;
        vsigma[ip] += tvsigma0;
        let t76 = t20 * t20;
        let t78 = 1.0 / t19 / t76;
        let t86 = t76 * t21;
        let t88 = 1.0 / t19 / t86;
        let t89 = sigma[ip] * sigma[ip];
        let t94 = t39 * t88;
        let t96 = 1.0 / t47 / t28;
        let t97 = t25 * t96;
        let t98 = t97 * t89;
        let t102 = piecewise3(t2, 0.0, t18 / t19 / rho[ip] * t25 * t29 / 6.0 - 0.014251428880416323 * t39 * t78 * sigma[ip] * t29 + 0.000805662725306196 * t39 * t78 * t50 + 6.78368014707817e-06 * t39 * t88 * t89 * t48 - 3.834954572457493e-07 * t94 * t98);
        let tv2rho20 = 2.0 * rho[ip] * t102 + 4.0 * t54;
        v2rho2[ip] += tv2rho20;
        let t108 = t76 * t20;
        let t110 = 1.0 / t19 / t108;
        let t111 = t110 * t48;
        let t115 = t41 * t25;
        let t119 = t39 * t110;
        let t120 = t97 * sigma[ip];
        let t124 = piecewise3(t2, 0.0, 0.00395873024456009 * t39 * t41 * t29 - 2.2612267156927235e-06 * t39 * t111 * sigma[ip] - 0.00022379520147394332 * t39 * t115 * t48 + 1.2783181908191643e-07 * t119 * t120);
        let tv2rhosigma0 = 2.0 * rho[ip] * t124 + 2.0 * t67;
        v2rhosigma[ip] += tv2rhosigma0;
        let t127 = t76 * rho[ip];
        let t129 = 1.0 / t19 / t127;
        let t133 = t129 * t25;
        let t138 = piecewise3(t2, 0.0, 7.537422385642411e-07 * t39 * t129 * t48 - 4.2610606360638806e-08 * t39 * t133 * t96);
        let tv2sigma20 = 2.0 * rho[ip] * t138;
        v2sigma2[ip] += tv2sigma20;
        let t151 = t76 * t76;
        let t153 = 1.0 / t19 / t151;
        let t158 = t39 * t153;
        let t163 = 1.0 / t19 / t151 / t21;
        let t164 = t89 * sigma[ip];
        let t169 = t39 * t163;
        let t170 = t47 * t47;
        let t171 = 1.0 / t170;
        let t172 = t25 * t171;
        let t173 = t172 * t164;
        let t177 = piecewise3(t2, 0.0, -t18 * t62 * t29 / 4.0 + 0.06294381088850542 * t39 * t129 * sigma[ip] * t29 - 0.003558343703435699 * t39 * t129 * t50 - 7.122864154432079e-05 * t39 * t153 * t89 * t48 + 4.026702301080368e-06 * t158 * t98 + 1.4530642875041441e-08 * t39 * t163 * t164 * t96 - 8.21447269420395e-10 * t169 * t173);
        let tv3rho30 = 2.0 * rho[ip] * t177 + 6.0 * t102;
        v3rho3[ip] += tv3rho30;
        let t184 = t88 * t48;
        let t190 = 1.0 / t19 / t151 / t20;
        let t191 = t190 * t96;
        let t201 = t39 * t190;
        let t202 = t172 * t89;
        let t206 = piecewise3(t2, 0.0, -0.013855555855960315 * t39 * t78 * t29 + 2.035104044123451e-05 * t39 * t184 * sigma[ip] - 4.8435476250138135e-09 * t39 * t191 * t89 + 0.0007832832051588017 * t39 * t78 * t25 * t48 - 1.1504863717372478e-06 * t94 * t120 + 2.73815756473465e-10 * t201 * t202);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t206 + 4.0 * t124;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t213 = 1.0 / t19 / t151 / rho[ip];
        let t214 = t213 * t96;
        let t222 = t39 * t213;
        let t223 = t172 * sigma[ip];
        let t227 = piecewise3(t2, 0.0, -4.145582312103326e-06 * t39 * t111 + 1.6145158750046046e-09 * t39 * t214 * sigma[ip] + 2.3435833498351344e-07 * t39 * t110 * t25 * t96 - 9.127191882448833e-11 * t222 * t223);
        let tv3rhosigma20 = 2.0 * rho[ip] * t227 + 2.0 * t138;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t238 = piecewise3(t2, 0.0, -5.381719583348681e-10 * t39 * t153 * t96 + 3.042397294149611e-11 * t39 * t153 * t25 * t171);
        let tv3sigma30 = 2.0 * rho[ip] * t238;
        v3sigma3[ip] += tv3sigma30;
    }
}
