//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1200/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1200(t23682: f64, t23620: f64, t23622: f64, t23624: f64, t23626: f64, t23630: f64, t23633: f64, t23635: f64, t23637: f64, t23640: f64, t23644: f64, t23660: f64) -> f64 {
    let t24776 = 0.17757530864197530864e0_f64 * t23682;
    let t24788 = t24776 - 0.45662222222222222221e-1_f64 * t23620 - 0.3044148148148148148e-1_f64 * t23622 + 0.22831111111111111111e-1_f64 * t23624 + 0.25367901234567901233e-1_f64 * t23626 - 0.50735802469135802467e-1_f64 * t23630 - 0.17123333333333333333e-1_f64 * t23633 + 0.71030123456790123454e-1_f64 * t23635 - 0.9132444444444444444e-1_f64 * t23637 + 0.2283111111111111111e0_f64 * t23640 + 0.10274e0_f64 * t23644 + 0.13698666666666666667e0_f64 * t23660;
    t24788
}
