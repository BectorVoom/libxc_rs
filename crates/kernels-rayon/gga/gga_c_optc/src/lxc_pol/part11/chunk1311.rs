//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1311/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1311(t39411: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49271: f64, t49273: f64, t56966: f64, t56969: f64, t56972: f64, t56975: f64, t56978: f64, t56981: f64, t56984: f64) -> f64 {
    let t57403 = -0.13772666666666666666e1_f64 * t39411 - 0.91817777777777777776e0_f64 * t39413 + 0.27545333333333333333e1_f64 * t39418 + 0.13772666666666666667e1_f64 * t49240 - 0.41318e1_f64 * t49242 - 0.166712e1_f64 * t49271 + 0.27785333333333333333e0_f64 * t49273 + 0.123954e2_f64 * t56966 - 0.34431666666666666667e1_f64 * t56969 - 0.13892666666666666667e0_f64 * t56972 - 0.27785333333333333334e0_f64 * t56975 - 0.185931e2_f64 * t56978 + 0.41318e1_f64 * t56981 - 0.13772666666666666667e1_f64 * t56984;
    t57403
}
