//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1365/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1365<F: Float>(t1307: F, t20916: F, t5709: F, t102237: F, t102240: F, t102245: F, t102262: F, t102292: F, t102299: F, t102303: F, t103340: F, t27369: F, t27459: F, t28369: F, t28439: F, t29289: F, t29314: F, t7908: F) -> (F, F) {
    let t103423 = t5709 * t20916 * t1307;
    let t103438 = -F::cast_from(0.16581944444444444444e-2_f64) * t102237 - F::cast_from(0.88437037037037037033e-2_f64) * t102240 - F::cast_from(0.33163888888888888888e-2_f64) * t102245 + F::cast_from(0.30918233506944444444e-4_f64) * t27369 * t103423 + F::cast_from(0.46336805555555555557e-3_f64) * t28369 * t28439 + F::cast_from(0.46336805555555555556e-3_f64) * t27459 * t29314 + F::cast_from(0.11054629629629629629e-2_f64) * t102262 - F::cast_from(0.13901041666666666667e-2_f64) * t27459 * t29289 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t103340 + F::cast_from(0.66327777777777777776e-2_f64) * t102292 - F::new(0.1492375e-1) * t102299 + F::cast_from(0.33163888888888888888e-2_f64) * t102303;
    (t103423, t103438)
}
