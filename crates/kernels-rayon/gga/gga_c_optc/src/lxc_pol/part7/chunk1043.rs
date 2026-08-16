//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1043/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1043(t1796: f64, t3648: f64, t603: f64, t22497: f64, t22562: f64, t22578: f64, t22581: f64, t22593: f64, t22681: f64, t22683: f64, t22685: f64, t22687: f64, t22690: f64, t22694: f64) -> (f64, f64) {
    let t22697 = 0.67471169937307261776e-1_f64 * t1796 * t3648 * t603;
    let t22698 = t22681 - t22683 - t22685 + t22687 - t22690 - t22694 - t22497 + t22562 + t22578 + t22581 - t22593 + t22697;
    (t22697, t22698)
}
