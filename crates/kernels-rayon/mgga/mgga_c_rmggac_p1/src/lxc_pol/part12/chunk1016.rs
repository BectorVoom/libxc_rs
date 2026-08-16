//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1016/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1016(t41276: f64, t8746: f64, t41209: f64, t8750: f64, t41212: f64, t41215: f64, t7603: f64, t41218: f64, t41221: f64, t41224: f64, t41227: f64, t8761: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41277 = t8746 * t41276;
    let t41279 = t8750 * t41209;
    let t41281 = t8750 * t41212;
    let t41283 = t7603 * t41215;
    let t41285 = t7603 * t41218;
    let t41287 = t7603 * t41221;
    let t41289 = t7603 * t41224;
    let t41291 = t8761 * t41227;
    (t41277, t41279, t41281, t41283, t41285, t41287, t41289, t41291)
}
