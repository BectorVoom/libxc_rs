//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 749/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk749<F: Float>(t160: F, t8593: F, t2315: F, t8581: F, t648: F, t8590: F, t4620: F, t15: F, t2317: F, t2320: F, t2444: F, t2448: F, t650: F, t720: F, t8573: F, t8574: F, t8578: F, t8585: F) -> (F, F, F, F) {
    let t8594 = t8593 * t160;
    let t8596 = t2315 * t8581;
    let t8598 = t648 * t8590;
    let t8601 = -F::cast_from(0.26426666666666666667e-1_f64) * t8594 + F::cast_from(0.17617777777777777778e-1_f64) * t8596 - F::cast_from(0.20554074074074074074e-1_f64) * t8598 - F::cast_from(0.12841111111111111111e-1_f64) * t4620;
    let t8604 = -t8573 * t8574 / F::cast_from(3.0_f64) - t8578 * t2317 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2444 * t8581 - t8585 * t650 / F::cast_from(4.0_f64) + t2448 * t2320 / F::cast_from(3.0_f64) - F::cast_from(7.0_f64) / F::cast_from(27.0_f64) * t720 * t8590 + t15 * t8601 / F::cast_from(2.0_f64);
    (t8594, t8596, t8598, t8604)
}
