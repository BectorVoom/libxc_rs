//! LDA_C_RC04 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rc04.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_rc04_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = pow_1_3(zeta_threshold);
        let t3 = t2 * t2;
        let t4 = piecewise3(1.0 <= zeta_threshold, t3, 1.0);
        let t5 = t4 * t4;
        let t6 = t5 * t4;
        let t7 = M_CBRT3;
        let t9 = pow_1_3(1.0 / M_PI);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t18 = 4.88827 + 0.79425925 * t10 * t12 / t13;
        let t19 = rmath::atan(t18);
        let t23 = t7 * t7;
        let t24 = t6 * (-0.655868 * t19 + 0.897889) * t23;
        let t26 = 1.0 / t9 * t11;
        let t28 = t24 * t26 * t13;
        let tzk0 = t28 / 3.0;
        zk[ip] += tzk0;
        let t30 = t18 * t18;
        let t31 = t30 + 1.0;
        let t32 = 1.0 / t31;
        let tvrho0 = 4.0 / 9.0 * t28 + 0.6945723010386666 * t6 * t32;
        vrho[ip] += tvrho0;
        let t39 = t13 * t13;
        let t44 = t31 * t31;
        let t45 = 1.0 / t44;
        let t46 = t6 * t45;
        let tv2rho20 = 0.9260964013848889 * t6 / rho[ip] * t32 + 4.0 / 27.0 * t24 * t26 / t39 + 0.3677803165958304 * t46 * t18 * t10 * t12 / t13 / rho[ip];
        v2rho2[ip] += tv2rho20;
        let t54 = rho[ip] * rho[ip];
        let t65 = t18 * t7 * t9 * t12;
        let t74 = 1.0 / t44 / t31;
        let t75 = t6 * t74;
        let t77 = t9 * t9;
        let t78 = t23 * t77;
        let t80 = 1.0 / t39 / t54;
        let t86 = t77 * t11;
        let tv3rho30 = -0.6173976009232592 * t6 / t54 * t32 - 1e-20 * t6 / t13 / t54 * t45 * t65 - 8.0 / 81.0 * t24 * t26 / t39 / rho[ip] + 1.5579355649288897 * t75 * t30 * t78 * t11 * t80 - 0.38948389123222243 * t46 * t23 * t86 * t80;
        v3rho3[ip] += tv3rho30;
    }
}
