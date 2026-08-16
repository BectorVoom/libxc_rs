//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1207/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1207(t28255: f64, t37294: f64, t47877: f64, t47879: f64, t4595: f64) -> (f64, f64, f64, f64, f64) {
    let t55878 = 0.4155781415850207192e3_f64 * t28255;
    let t55882 = 0.14649244029402527953e-2_f64 * t37294;
    let t55883 = 16.0_f64 * t47877;
    let t55884 = 16.0_f64 * t47879;
    let t55893 = t4595 * t4595;
    (t55878, t55882, t55883, t55884, t55893)
}
