//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 231/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk231(t135: f64, t626: f64, t628: f64, t631: f64, t636: f64, t648: f64, t656: f64, t661: f64) -> f64 {
    let t664 = -t626 - t628 * t631 / 48.0_f64 - 0.27166129655589868296e-2_f64 * t636 * t648 - t656 - 0.10866451862235947318e-1_f64 * t135 * t661;
    t664
}
