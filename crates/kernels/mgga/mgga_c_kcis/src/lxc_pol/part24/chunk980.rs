//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 980/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk980<F: Float>(t20562: F, t20598: F, t20630: F, t20669: F, t1268: F, t1239: F, t6835: F, t10450: F, t1240: F, t1269: F, t14065: F, t14102: F, t14390: F, t15632: F, t15638: F, t15639: F, t15648: F, t19743: F, t19747: F, t19752: F, t19754: F, t19759: F, t19766: F, t19771: F, t20294: F, t3644: F, t5342: F, t5345: F) -> (F, F, F) {
    let t20671 = t20562 + t20598 + t20630 + t20669;
    let t20672 = t20671 * t1268;
    let t20684 = t6835 * t1239;
    let t20689 = F::cast_from(0.77382407407407407407e-3_f64) * t19743 + F::cast_from(0.12897067901234567901e-2_f64) * t19747 - F::cast_from(0.77382407407407407407e-3_f64) * t14065 + F::cast_from(0.46429444444444444444e-2_f64) * t19752 - F::new(0.66725e-1) * t1240 * t20672 + F::cast_from(0.11607361111111111111e-2_f64) * t19754 - t15632 - F::new(0.13345e0) * t5345 * t5342 - F::cast_from(0.38691203703703703703e-3_f64) * t19759 - t15638 - t15639 + F::cast_from(0.46429444444444444444e-2_f64) * t14102 - t15648 - F::cast_from(0.11607361111111111111e-2_f64) * t19766 + F::cast_from(0.61905925925925925925e-2_f64) * t19771 + F::cast_from(0.178089025e-1_f64) * t3644 * t20294 - F::new(0.66725e-1) * t20684 * t1269 - F::cast_from(0.38691203703703703703e-3_f64) * t10450 - F::cast_from(0.51588271604938271603e-3_f64) * t14390;
    (t20671, t20684, t20689)
}
