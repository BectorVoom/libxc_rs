//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 971/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk971(t41218: f64, t7603: f64, t41221: f64, t41224: f64, t41227: f64, t8761: f64, t41276: f64, t1635: f64, t2084: f64, t8746: f64, t1624: f64, t8764: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41285 = t7603 * t41218;
    let t41287 = t7603 * t41221;
    let t41289 = t7603 * t41224;
    let t41291 = t8761 * t41227;
    let t41294 = t8761 * t41276;
    let t41296 = t2084 * t1635;
    let t41297 = t8746 * t41296;
    let t41299 = t8761 * t41296;
    let t41301 = t2084 * t1624;
    let t41302 = t8764 * t41301;
    (t41285, t41287, t41289, t41291, t41294, t41297, t41299, t41301, t41302)
}
