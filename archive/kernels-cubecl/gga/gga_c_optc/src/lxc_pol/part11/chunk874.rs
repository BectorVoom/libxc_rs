//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 874/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk874<F: Float>(t10188: F, t13699: F, t13701: F, t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16650: F, t7713: F, t232: F) -> (F, F) {
    let t16652 = -t7713 - F::cast_from(0.23744444444444444444e-1_f64) * t10188 + F::cast_from(0.11872222222222222222e-1_f64) * t13699 - F::cast_from(0.35616666666666666666e-1_f64) * t13701 + F::cast_from(0.17808333333333333333e-1_f64) * t13703 - F::cast_from(0.19787037037037037037e-1_f64) * t16630 + F::cast_from(0.71233333333333333332e-1_f64) * t16634 - F::cast_from(0.35616666666666666666e-1_f64) * t16638 - F::cast_from(0.10685e0_f64) * t16642 + F::cast_from(0.10685e0_f64) * t16646 - F::cast_from(0.17808333333333333333e-1_f64) * t16650;
    let t16654 = F::cast_from(0.62182e-1_f64) * t16652 * t232;
    (t16652, t16654)
}
