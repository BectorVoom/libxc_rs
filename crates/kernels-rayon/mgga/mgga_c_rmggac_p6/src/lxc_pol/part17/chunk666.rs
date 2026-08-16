//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 666/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk666(t1997: f64, t9222: f64, t2057: f64, t5055: f64, t530: f64, t7894: f64, t1550: f64, t9005: f64, t2406: f64, t275: f64, t1668: f64, t2131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9223 = t9222 * t1997;
    let t9225 = t5055 * t2057;
    let t9227 = t530 * t7894;
    let t9229 = t1550 * t9005;
    let t9231 = t275 * t2406;
    let t9232 = t1668 * t2131;
    (t9223, t9225, t9227, t9229, t9231, t9232)
}
