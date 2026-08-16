//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1172/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1172(t23682: f64, t23620: f64, t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23660: f64) -> f64 {
    let t24321 = 0.96141975308641975307e-1_f64 * t23682;
    let t24333 = t24321 - 0.24722222222222222222e-1_f64 * t23620 - 0.16481481481481481482e-1_f64 * t23622 + 0.12361111111111111111e-1_f64 * t23624 + 0.13734567901234567901e-1_f64 * t23626 - 0.27469135802469135803e-1_f64 * t23630 - 0.92708333333333333333e-2_f64 * t23633 + 0.38456790123456790123e-1_f64 * t23635 - 0.49444444444444444444e-1_f64 * t23637 + 0.12361111111111111111e0_f64 * t23640 + 0.55625000000000000001e-1_f64 * t23644 + 0.74166666666666666668e-1_f64 * t23660;
    t24333
}
