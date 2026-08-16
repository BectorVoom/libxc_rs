//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 980/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk980(t20562: f64, t20598: f64, t20630: f64, t20669: f64, t1268: f64, t1239: f64, t6835: f64, t10450: f64, t1240: f64, t1269: f64, t14065: f64, t14102: f64, t14390: f64, t15632: f64, t15638: f64, t15639: f64, t15648: f64, t19743: f64, t19747: f64, t19752: f64, t19754: f64, t19759: f64, t19766: f64, t19771: f64, t20294: f64, t3644: f64, t5342: f64, t5345: f64) -> (f64, f64, f64) {
    let t20671 = t20562 + t20598 + t20630 + t20669;
    let t20672 = t20671 * t1268;
    let t20684 = t6835 * t1239;
    let t20689 = 0.77382407407407407407e-3_f64 * t19743 + 0.12897067901234567901e-2_f64 * t19747 - 0.77382407407407407407e-3_f64 * t14065 + 0.46429444444444444444e-2_f64 * t19752 - 0.66725e-1_f64 * t1240 * t20672 + 0.11607361111111111111e-2_f64 * t19754 - t15632 - 0.13345e0_f64 * t5345 * t5342 - 0.38691203703703703703e-3_f64 * t19759 - t15638 - t15639 + 0.46429444444444444444e-2_f64 * t14102 - t15648 - 0.11607361111111111111e-2_f64 * t19766 + 0.61905925925925925925e-2_f64 * t19771 + 0.178089025e-1_f64 * t3644 * t20294 - 0.66725e-1_f64 * t20684 * t1269 - 0.38691203703703703703e-3_f64 * t10450 - 0.51588271604938271603e-3_f64 * t14390;
    (t20671, t20684, t20689)
}
