//! MGGA_C_CS kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cs.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_cs_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = pow_1_3(rho[ip]);
        let t3 = 1.0 / t2;
        let t5 = 1.0 + 0.349 * t3;
        let t6 = 1.0 / t5;
        let t8 = rmath::exp(-0.2533 * t3);
        let t10 = zeta_threshold * zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = t11 * t11;
        let t14 = piecewise3(1.0 <= zeta_threshold, t12 * t10, 1.0);
        let t15 = M_CBRT2;
        let t16 = t14 * t15;
        let t17 = t15 * t15;
        let t18 = tau[ip] * t17;
        let t19 = t2 * t2;
        let t21 = 1.0 / t19 / rho[ip];
        let t23 = lapl[ip] * t17;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t19 / t29;
        let t36 = t16 * (t18 * t21 - t23 * t21 / 8.0) / 4.0 - sigma[ip] * t31 / 8.0 + lapl[ip] * t21 / 8.0;
        let t39 = 1.0 + 0.264 * t8 * t36;
        let tzk0 = -0.04918 * t6 * t39;
        zk[ip] += tzk0;
        let t42 = t5 * t5;
        let t43 = 1.0 / t42;
        let t44 = t3 * t43;
        let t47 = rho[ip] * t6;
        let t49 = 1.0 / t2 / rho[ip];
        let t50 = t49 * t8;
        let t60 = t29 * rho[ip];
        let t62 = 1.0 / t19 / t60;
        let t67 = t16 * (-5.0 / 3.0 * t18 * t31 + 5.0 / 24.0 * t23 * t31) / 4.0 + sigma[ip] * t62 / 3.0 - 5.0 / 24.0 * lapl[ip] * t31;
        let t70 = 0.0222904 * t50 * t36 + 0.264 * t8 * t67;
        let tvrho0 = tzk0 - 0.005721273333333333 * t44 * t39 - 0.04918 * t47 * t70;
        vrho[ip] += tvrho0;
        let t73 = t21 * t6;
        let tvsigma0 = 0.00162294 * t73 * t8;
        vsigma[ip] += tvsigma0;
        let t78 = -t14 * t21 / 16.0 + t21 / 8.0;
        let t79 = t8 * t78;
        let tvlapl0 = -0.01298352 * t47 * t79;
        vlapl[ip] += tvlapl0;
        let t84 = t8 * t14;
        let tvtau0 = -0.00649176 / t19 * t6 * t84;
        vtau[ip] += tvtau0;
        let t87 = t43 * t39;
        let t93 = 1.0 / t42 / t5;
        let t94 = t21 * t93;
        let t100 = 1.0 / t2 / t29;
        let t101 = t100 * t8;
        let t104 = t31 * t8;
        let t116 = t29 * t29;
        let t118 = 1.0 / t19 / t116;
        let t123 = t16 * (40.0 / 9.0 * t18 * t62 - 5.0 / 9.0 * t23 * t62) / 4.0 - 11.0 / 9.0 * sigma[ip] * t118 + 5.0 / 9.0 * lapl[ip] * t62;
        let t126 = -0.029720533333333334 * t101 * t36 + 0.0018820527733333333 * t104 * t36 + 0.0445808 * t50 * t67 + 0.264 * t8 * t123;
        let tv2rho20 = -0.003814182222222222 * t87 * t49 - 0.09836 * t6 * t70 - 0.0013311495955555556 * t94 * t39 - 0.011442546666666666 * t44 * t70 - 0.04918 * t47 * t126;
        v2rho2[ip] += tv2rho20;
        let t129 = t31 * t6;
        let t132 = 1.0 / t60;
        let t133 = t132 * t43;
        let t136 = t132 * t6;
        let tv2rhosigma0 = -0.0027049 * t129 * t8 + 0.00018880202 * t133 * t8 + 0.000137030234 * t136 * t8;
        v2rhosigma[ip] += tv2rhosigma0;
        let t139 = t6 * t8;
        let t144 = t3 * t6;
        let t150 = 5.0 / 48.0 * t14 * t31 - 5.0 / 24.0 * t31;
        let t151 = t8 * t150;
        let tv2rholapl0 = -0.01298352 * t139 * t78 - 0.00151041616 * t44 * t79 - 0.001096241872 * t144 * t79 - 0.01298352 * t47 * t151;
        v2rholapl[ip] += tv2rholapl0;
        let t156 = 1.0 / t29;
        let tv2rhotau0 = 0.00432784 * t73 * t84 - 0.00075520808 * t156 * t43 * t84 - 0.000548120936 * t156 * t6 * t84;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t163 = t93 * t39;
        let t166 = t43 * t70;
        let t173 = t42 * t42;
        let t174 = 1.0 / t173;
        let t175 = t132 * t174;
        let t183 = 1.0 / t2 / t60;
        let t184 = t183 * t8;
        let t187 = t62 * t8;
        let t192 = 1.0 / t116;
        let t193 = t192 * t8;
        let t207 = t116 * rho[ip];
        let t209 = 1.0 / t19 / t207;
        let t214 = t16 * (-440.0 / 27.0 * t18 * t118 + 55.0 / 27.0 * t23 * t118) / 4.0 + 154.0 / 27.0 * sigma[ip] * t209 - 55.0 / 27.0 * lapl[ip] * t118;
        let t217 = 0.0693479111111111 * t184 * t36 - 0.007528211093333333 * t187 * t36 - 0.0891616 * t101 * t67 + 0.00015890798916177778 * t193 * t36 + 0.00564615832 * t104 * t67 + 0.0668712 * t50 * t123 + 0.264 * t8 * t214;
        let tv3rho30 = 0.0013311495955555556 * t163 * t31 - 0.011442546666666666 * t166 * t49 + 0.005085576296296296 * t87 * t100 - 0.14754 * t6 * t126 - 0.0004645712088488889 * t175 * t39 - 0.003993448786666667 * t94 * t70 - 0.01716382 * t44 * t126 - 0.04918 * t47 * t217;
        v3rho3[ip] += tv3rho30;
        let t220 = t62 * t6;
        let t223 = t192 * t43;
        let t226 = t192 * t6;
        let t230 = 1.0 / t2 / t116;
        let t231 = t230 * t93;
        let t234 = t230 * t43;
        let t237 = t230 * t6;
        let tv3rho2sigma0 = 0.007213066666666667 * t220 * t8 - 0.0008810760933333333 * t223 * t8 - 0.0006394744253333333 * t226 * t8 + 4.392793665333333e-05 * t231 * t8 + 3.188236777733333e-05 * t234 * t8 + 1.1569919424066667e-05 * t237 * t8;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t240 = t43 * t8;
        let t244 = t6 * t49;
        let t251 = t21 * t43;
        let t263 = -5.0 / 18.0 * t14 * t62 + 5.0 / 9.0 * t62;
        let t264 = t8 * t263;
        let tv3rho2lapl0 = -0.0010069441066666667 * t240 * t78 * t49 - 0.0007308279146666667 * t244 * t79 - 0.02596704 * t139 * t150 - 0.00035142349322666666 * t94 * t79 - 0.00025505894221866665 * t251 * t79 - 0.00302083232 * t44 * t151 - 9.255935539253333e-05 * t73 * t79 - 0.002192483744 * t144 * t151 - 0.01298352 * t47 * t264;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = -0.007213066666666667 * t129 * t84 + 0.0020138882133333333 * t133 * t84 + 0.0014616558293333334 * t136 * t84 - 0.00017571174661333333 * t183 * t93 * t84 - 0.00012752947110933333 * t183 * t43 * t84 - 4.627967769626667e-05 * t183 * t6 * t84;
        v3rho2tau[ip] += tv3rho2tau0;
        let tv3rhosigma20 = 0.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
    }
}
