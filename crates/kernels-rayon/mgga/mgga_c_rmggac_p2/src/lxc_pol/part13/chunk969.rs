//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 969/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk969(t3839: f64, t39055: f64, t3826: f64, t39059: f64, t41031: f64, t854: f64, t41047: f64, t797: f64, t25529: f64, t36: f64, t5169: f64, t41027: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41243 = t3839 * t39055;
    let t41245 = t3826 * t39059;
    let t41247 = t854 * t41031;
    let t41255 = t854 * t41047;
    let t41257 = t797 * t41031;
    let t41262 = t25529 * t36;
    let t41263 = t41262 * t5169;
    let t41265 = t851 * t41027;
    (t41243, t41245, t41247, t41255, t41257, t41263, t41265)
}
