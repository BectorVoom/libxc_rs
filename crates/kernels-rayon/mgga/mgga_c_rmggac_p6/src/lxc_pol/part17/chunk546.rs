//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 546/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk546(t31: f64, t7352: f64, t7351: f64, t7349: f64, t1179: f64, t214: f64, t1968: f64, t1966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7353 = t7352 * t31;
    let t7354 = t7351 * t7353;
    let t7355 = t7349 * t7354;
    let t7363 = t1179 * t214;
    let t7364 = t7363 * t1968;
    let t7365 = t1966 * t7364;
    (t7353, t7354, t7355, t7363, t7364, t7365)
}
