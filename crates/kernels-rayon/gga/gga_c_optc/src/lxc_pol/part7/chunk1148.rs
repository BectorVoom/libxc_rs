//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1148/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1148(t211: f64, t217: f64, t22502: f64, t23845: f64, t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23647: f64, t23651: f64, t23653: f64, t23655: f64) -> (f64, f64) {
    let t23913 = 1.0_f64 / t217 / t22502 / t211 / 96.0_f64;
    let t23914 = t23913 * t23845;
    let t23918 = -0.53675555555555555556e0_f64 * t23622 + 0.40256666666666666668e0_f64 * t23624 + 0.44729629629629629629e0_f64 * t23626 - 0.89459259259259259259e0_f64 * t23630 - 0.301925e0_f64 * t23633 + 0.12524296296296296297e1_f64 * t23635 - 0.16102666666666666667e1_f64 * t23637 + 0.40256666666666666666e1_f64 * t23640 + 0.181155e1_f64 * t23644 + 0.198684e1_f64 * t23647 + 0.49671e0_f64 * t23651 - 0.485484375e1_f64 * t23914 - 0.24154e1_f64 * t23653 + 0.80513333333333333333e0_f64 * t23655;
    (t23914, t23918)
}
