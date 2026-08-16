//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1194/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1194(t23620: f64, t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23660: f64, t24678: f64) -> f64 {
    let t24690 = t24678 - 0.47488888888888888888e-1_f64 * t23620 - 0.31659259259259259258e-1_f64 * t23622 + 0.23744444444444444444e-1_f64 * t23624 + 0.26382716049382716049e-1_f64 * t23626 - 0.52765432098765432099e-1_f64 * t23630 - 0.17808333333333333333e-1_f64 * t23633 + 0.73871604938271604937e-1_f64 * t23635 - 0.94977777777777777776e-1_f64 * t23637 + 0.23744444444444444444e0_f64 * t23640 + 0.10685e0_f64 * t23644 + 0.14246666666666666667e0_f64 * t23660;
    t24690
}
