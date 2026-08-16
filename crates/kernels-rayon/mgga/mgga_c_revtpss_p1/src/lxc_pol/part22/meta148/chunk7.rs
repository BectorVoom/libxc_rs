//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 985/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk985(t1161: f64, t1170: f64, t1180: f64, t1189: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3444: f64, t3447: f64, t3452: f64, t3454: f64, t3472: f64, t3477: f64, t3480: f64, t3489: f64, t3491: f64, t3496: f64, t3498: f64, t3516: f64, t3521: f64, t3524: f64, t435: f64) -> f64 {
    let t3527 = -0.310907e-1_f64 * t3444 * t435 + 2.0_f64 * t3447 * t1170 - 2.0_f64 * t3452 * t3454 + 1.0_f64 * t1161 * t3472 + 0.32163958997385070134e2_f64 * t3477 * t3480 + t3378 - t3381 + t3388 - t3430 - t3438 - 0.19751673498613801407e-1_f64 * t3489 + 0.11696447245269292414e1_f64 * t3491 * t1189 - 0.11696447245269292414e1_f64 * t3496 * t3498 + 0.5848223622634646207e0_f64 * t1180 * t3516 + 0.17315859105681463759e2_f64 * t3521 * t3524;
    t3527
}
