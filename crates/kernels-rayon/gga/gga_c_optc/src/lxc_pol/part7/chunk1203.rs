//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1203/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1203(t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23647: f64, t23651: f64, t23653: f64, t23655: f64, t23914: f64) -> f64 {
    let t24855 = -0.91817777777777777776e0_f64 * t23622 + 0.68863333333333333332e0_f64 * t23624 + 0.76514814814814814814e0_f64 * t23626 - 0.15302962962962962963e1_f64 * t23630 - 0.516475e0_f64 * t23633 + 0.21424148148148148148e1_f64 * t23635 - 0.27545333333333333333e1_f64 * t23637 + 0.68863333333333333334e1_f64 * t23640 + 0.309885e1_f64 * t23644 + 0.250068e1_f64 * t23647 + 0.62517e0_f64 * t23651 - 0.6618234375e1_f64 * t23914 - 0.41318e1_f64 * t23653 + 0.13772666666666666666e1_f64 * t23655;
    t24855
}
