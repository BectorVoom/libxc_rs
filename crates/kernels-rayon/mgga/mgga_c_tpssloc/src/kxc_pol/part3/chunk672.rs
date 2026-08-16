//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 672/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk672(t3377: f64, t3403: f64, t1129: f64, t1138: f64, t1148: f64, t1157: f64, t3258: f64, t3261: f64, t3268: f64, t3310: f64, t3318: f64, t3324: f64, t3327: f64, t3332: f64, t3334: f64, t3352: f64, t3357: f64, t3360: f64, t3369: f64, t3371: f64, t3376: f64, t3378: f64, t3396: f64, t3401: f64, t436: f64) -> (f64, f64) {
    let t3404 = t3377 * t3403;
    let t3407 = -0.310907e-1_f64 * t3324 * t436 + 2.0_f64 * t3327 * t1138 - 2.0_f64 * t3332 * t3334 + 1.0_f64 * t1129 * t3352 + 0.32163958997385070134e2_f64 * t3357 * t3360 + t3258 - t3261 + t3268 - t3310 - t3318 - 0.19751673498613801407e-1_f64 * t3369 + 0.11696447245269292414e1_f64 * t3371 * t1157 - 0.11696447245269292414e1_f64 * t3376 * t3378 + 0.5848223622634646207e0_f64 * t1148 * t3396 + 0.17315859105681463759e2_f64 * t3401 * t3404;
    (t3404, t3407)
}
