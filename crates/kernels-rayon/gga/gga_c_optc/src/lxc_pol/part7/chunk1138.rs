//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1138/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1138(t23620: f64, t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23647: f64, t23682: f64) -> (f64, f64) {
    let t23758 = -0.19384444444444444445e4_f64 * t23620 - 0.12922962962962962963e4_f64 * t23622 + 0.96922222222222222224e3_f64 * t23624 + 0.10769135802469135803e4_f64 * t23626 - 0.21538271604938271605e4_f64 * t23630 - 0.72691666666666666667e3_f64 * t23633 + 0.30153580246913580247e4_f64 * t23635 - 0.38768888888888888889e4_f64 * t23637 + 0.96922222222222222221e4_f64 * t23640 + 0.43614999999999999999e4_f64 * t23644 + 1888.0_f64 * t23647;
    let t23769 = 0.75383950617283950617e4_f64 * t23682;
    (t23758, t23769)
}
