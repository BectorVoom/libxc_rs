//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 913/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk913(t75516: f64, t7788: f64, t69463: f64, t69465: f64, t69469: f64, t25820: f64, t75836: f64, t25877: f64, t75839: f64, t25854: f64, t75842: f64, t14327: f64, t15093: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76381 = t7788 * t75516;
    let t76414 = 0.15965655602485078085e0_f64 * t69463;
    let t76415 = 0.15965655602485078085e0_f64 * t69465;
    let t76416 = 0.23948483403727617128e0_f64 * t69469;
    let t76425 = 0.17961362552795712846e0_f64 * t25820 * t75836;
    let t76427 = 0.35922725105591425692e0_f64 * t25877 * t75839;
    let t76429 = 0.17961362552795712846e0_f64 * t25854 * t75842;
    let t76435 = t15093 * t14327;
    (t76381, t76414, t76415, t76416, t76425, t76427, t76429, t76435)
}
