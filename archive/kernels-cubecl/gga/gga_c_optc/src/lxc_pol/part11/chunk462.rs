//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 462/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk462<F: Float>(t1198: F, t484: F, t481: F, t2843: F, t2865: F, t474: F, t1084: F, t411: F) -> (F, F, F, F, F, F, F, F) {
    let t2885 = F::cast_from(1.0_f64) / t1198 / t484;
    let t2886 = t481 * t2885;
    let t2890 = F::cast_from(0.96922222222222222222e3_f64) * t2843;
    let t2895 = F::cast_from(0.13111111111111111111e3_f64) * t2865;
    let t2910 = t474 * t474;
    let t2911 = F::cast_from(1.0_f64) / t2910;
    let t2915 = t1084 * t411;
    let t2916 = F::cast_from(1.0_f64) / t2915;
    (t2885, t2886, t2890, t2895, t2910, t2911, t2915, t2916)
}
