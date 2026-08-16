//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 593/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk593(t4868: f64, t818: f64, t2500: f64, t2507: f64, t3640: f64, t3687: f64, t4770: f64, t4774: f64, t4778: f64, t4787: f64, t4794: f64, t4800: f64, t4802: f64, t4806: f64, t4809: f64, t4812: f64) -> (f64, f64) {
    let t4869 = t4868 * t818;
    let t4884 = -0.17648625e1_f64 * t4787 + 0.3529725e1_f64 * t4794 + t2500 + 0.34431666666666666666e0_f64 * t3640 - 0.34431666666666666667e0_f64 * t4770 + 0.103295e1_f64 * t4774 - 0.516475e0_f64 * t4778 + 0.31558125e0_f64 * t4800 + 0.6311625e0_f64 * t4802 + t2507 + 0.13892666666666666667e0_f64 * t3687 - 0.34731666666666666667e-1_f64 * t4806 + 0.20839e0_f64 * t4809 - 0.104195e0_f64 * t4812;
    (t4869, t4884)
}
