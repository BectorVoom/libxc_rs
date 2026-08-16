//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1197/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1197(t17697: f64, t8487: f64, t9169: f64, t9171: f64, t3109: f64, t9175: f64, t15911: f64, t4488: f64, t54753: f64, t935: f64, t1239: f64, t1506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55037 = t8487 * t17697;
    let t55039 = t9169 * t55037 * t9171;
    let t55042 = t9175 * t55037 * t3109;
    let t55044 = t15911 * t4488;
    let t55067 = t54753 * t935;
    let t55127 = t1239 * t1506;
    (t55037, t55039, t55042, t55044, t55067, t55127)
}
