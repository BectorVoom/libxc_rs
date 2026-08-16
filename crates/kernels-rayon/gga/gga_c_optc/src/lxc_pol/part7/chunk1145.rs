//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1145/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1145(t23682: f64, t23620: f64, t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23660: f64) -> f64 {
    let t23860 = 280.0_f64 / 81.0_f64 * t23682;
    let t23872 = t23860 - 8.0_f64 / 9.0_f64 * t23620 - 16.0_f64 / 27.0_f64 * t23622 + 4.0_f64 / 9.0_f64 * t23624 + 40.0_f64 / 81.0_f64 * t23626 - 80.0_f64 / 81.0_f64 * t23630 - t23633 / 3.0_f64 + 112.0_f64 / 81.0_f64 * t23635 - 16.0_f64 / 9.0_f64 * t23637 + 40.0_f64 / 9.0_f64 * t23640 + 2.0_f64 * t23644 + 8.0_f64 / 3.0_f64 * t23660;
    t23872
}
