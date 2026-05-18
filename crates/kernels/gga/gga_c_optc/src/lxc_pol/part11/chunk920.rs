//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 920/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk920<F: Float>(t10348: F, t13649: F, t13651: F, t13653: F, t16650: F, t16747: F, t16750: F, t16763: F, t16766: F, t8319: F, t8321: F, t10188: F, t13699: F, t13701: F, t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16756: F, t16759: F) -> (F, F) {
    let t17263 = F::new(0.821e-3) * t13649 - F::new(0.4926e-2) * t13651 + F::new(0.2463e-2) * t13653 - t8319 - F::new(0.19388333333333333333e1) * t16650 - t8321 - F::new(0.7389e-2) * t16747 + F::new(0.7389e-2) * t16763 + F::new(0.2463e-2) * t16750 - F::new(0.12315e-2) * t16766 - F::new(0.4105e-2) * t10348;
    let t17275 = F::new(0.12925555555555555555e1) * t13699 - F::new(0.38776666666666666665e1) * t13701 + F::new(0.19388333333333333333e1) * t13703 - F::new(0.2585111111111111111e1) * t10188 + F::new(0.77553333333333333331e1) * t16634 - F::new(0.38776666666666666665e1) * t16638 - F::new(0.11633e2) * t16642 + F::new(0.11633e2) * t16646 - F::new(0.21542592592592592592e1) * t16630 - F::new(0.54733333333333333333e-3) * t16756 - F::new(0.12315e-2) * t16759;
    (t17263, t17275)
}
