//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1047/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1047(t1982: f64, t2314: f64, t35512: f64, t118: f64, t128: f64, t2001: f64, t5738: f64, t675: f64, t2289: f64, t7921: f64, t333: f64, t3351: f64, t511: f64, t9210: f64, t9211: f64) -> (f64, f64, f64, f64) {
    let t41767 = t2314 * t35512 * t1982;
    let t41772 = t675 * t2001 * t118 * t128 * t5738;
    let t41774 = t7921 * t2289;
    let t41779 = t3351 * t9210 * t511 * t9211 * t333;
    (t41767, t41772, t41774, t41779)
}
