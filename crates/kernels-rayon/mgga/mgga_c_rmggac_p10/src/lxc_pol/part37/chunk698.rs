//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 698/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk698(t3102: f64, t35206: f64, t3851: f64, t69239: f64, t25518: f64, t3068: f64, t3826: f64, t3839: f64, t69211: f64, t68741: f64, t793: f64, t1326: f64, t14309: f64, t2048: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69289 = t3102 * t35206;
    let t69294 = t3851 * t69239;
    let t69296 = t25518 * t3068;
    let t69303 = t3826 * t69239;
    let t69313 = t3839 * t69211;
    let t69404 = t793 * t68741;
    let t69417 = t14309 * t1326 * t2048 * t352;
    (t69289, t69294, t69296, t69303, t69313, t69404, t69417)
}
