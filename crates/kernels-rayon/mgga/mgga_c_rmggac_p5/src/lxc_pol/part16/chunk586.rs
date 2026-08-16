//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 586/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk586(t1411: f64, t2011: f64, t291: f64, t2010: f64, t1661: f64, t2012: f64, t2020: f64, t2339: f64, t2019: f64, t1665: f64, t2323: f64, t2415: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8342 = t2011 * t1411;
    let t8343 = t8342 * t291;
    let t8344 = t2010 * t8343;
    let t8346 = t2012 * t1661;
    let t8347 = t2010 * t8346;
    let t8349 = t2020 * t2339;
    let t8350 = t2019 * t8349;
    let t8352 = t2012 * t1665;
    let t8353 = t2010 * t8352;
    let t8355 = t2020 * t2323;
    let t8356 = t2019 * t8355;
    let t8358 = t2415 * t935;
    (t8342, t8343, t8344, t8346, t8347, t8349, t8350, t8352, t8353, t8355, t8356, t8358)
}
