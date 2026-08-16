//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1169/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1169(t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23647: f64, t23651: f64, t23653: f64, t23655: f64, t23914: f64) -> f64 {
    let t24279 = -0.5314962962962962963e0_f64 * t23622 + 0.39862222222222222223e0_f64 * t23624 + 0.44291358024691358024e0_f64 * t23626 - 0.88582716049382716048e0_f64 * t23630 - 0.29896666666666666667e0_f64 * t23633 + 0.12401580246913580247e1_f64 * t23635 - 0.15944888888888888889e1_f64 * t23637 + 0.39862222222222222223e1_f64 * t23640 + 0.17938e1_f64 * t23644 + 0.197176e1_f64 * t23647 + 0.49293999999999999999e0_f64 * t23651 - 0.3560484375e1_f64 * t23914 - 0.23917333333333333333e1_f64 * t23653 + 0.79724444444444444444e0_f64 * t23655;
    t24279
}
