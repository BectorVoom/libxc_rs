//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3219/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3219(t16507: f64, t1858: f64, t3: f64, t5364: f64, t5381: f64, t55368: f64, t55374: f64, t55376: f64, t55378: f64, t580: f64, t66937: f64, t66976: f64, t66987: f64, t66989: f64, t66991: f64) -> f64 {
    let t66993 = t3 * t580 * t66937 + 2.0_f64 * t16507 * t1858 + 4.0_f64 * t5364 * t5381 + 2.0_f64 * t55368 + 2.0_f64 * t55374 + 4.0_f64 * t55376 + 4.0_f64 * t55378 + 2.0_f64 * t66976 + 2.0_f64 * t66987 + 2.0_f64 * t66989 + 4.0_f64 * t66991;
    t66993
}
