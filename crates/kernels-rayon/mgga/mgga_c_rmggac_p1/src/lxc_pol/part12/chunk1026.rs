//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1026/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1026(t35959: f64, t3839: f64, t5156: f64, t649: f64, t25640: f64, t40998: f64, t41150: f64, t3851: f64, t5260: f64, t35960: f64, t5263: f64, t2402: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t41400 = t3839 * t35959;
    let t41402 = t41400 * t649 * t5156;
    let t41404 = t25640 * t40998;
    let t41405 = t41404 * t41150;
    let t41407 = t3851 * t35959;
    let t41409 = t41407 * t649 * t5260;
    let t41412 = t35960 * t649 * t5263;
    let t41414 = t2402 * t848;
    (t41402, t41405, t41409, t41412, t41414)
}
