//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 839/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk839(t2100: f64, t41056: f64, t2103: f64, t41036: f64, t2118: f64, t35959: f64, t3839: f64, t25640: f64, t40998: f64, t3851: f64, t39696: f64, t5259: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41377 = t2100 * t41056;
    let t41378 = 0.18183107769496894486e-1_f64 * t41377;
    let t41379 = t2103 * t41036;
    let t41380 = 0.24244143692662525982e-1_f64 * t41379;
    let t41381 = t2118 * t41036;
    let t41400 = t3839 * t35959;
    let t41404 = t25640 * t40998;
    let t41407 = t3851 * t35959;
    let t41438 = t5259 * t39696;
    (t41378, t41380, t41381, t41400, t41404, t41407, t41438)
}
