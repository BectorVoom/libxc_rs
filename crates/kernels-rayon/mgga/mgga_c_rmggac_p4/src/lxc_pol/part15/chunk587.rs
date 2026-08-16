//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 587/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk587(t1665: f64, t2012: f64, t2010: f64, t2020: f64, t2323: f64, t2019: f64, t2415: f64, t935: f64, t938: f64, t623: f64, t880: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8352 = t2012 * t1665;
    let t8353 = t2010 * t8352;
    let t8355 = t2020 * t2323;
    let t8356 = t2019 * t8355;
    let t8358 = t2415 * t935;
    let t8359 = t2010 * t8358;
    let t8362 = t2415 * t938;
    let t8363 = t2010 * t8362;
    let t8365 = t623 * t880;
    (t8352, t8353, t8355, t8356, t8358, t8359, t8362, t8363, t8365)
}
