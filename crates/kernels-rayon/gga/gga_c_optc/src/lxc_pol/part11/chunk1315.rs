//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1315/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1315(t4884: f64, t39411: f64, t49385: f64, t49387: f64, t56966: f64, t56978: f64, t56981: f64, t56984: f64, t57024: f64, t57057: f64, t57060: f64, t57063: f64) -> (f64, f64) {
    let t57453 = t4884 * t4884;
    let t57501 = -0.17123333333333333333e-1_f64 * t57057 + 0.41096e0_f64 * t57060 - 0.61644e0_f64 * t56978 + 0.10274e0_f64 * t57063 - 0.9132444444444444444e-1_f64 * t49385 + 0.13698666666666666667e0_f64 * t49387 + 0.13698666666666666667e0_f64 * t56981 - 0.4566222222222222222e-1_f64 * t56984 - 0.45662222222222222221e-1_f64 * t39411 - 0.41095999999999999999e0_f64 * t57024 + 0.41095999999999999998e0_f64 * t56966;
    (t57453, t57501)
}
