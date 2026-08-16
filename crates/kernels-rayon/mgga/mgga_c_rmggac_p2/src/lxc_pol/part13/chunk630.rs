//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 630/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk630(t1540: f64, t511: f64, t650: f64, t1411: f64, t2011: f64, t291: f64, t2010: f64, t1661: f64, t2012: f64, t2020: f64, t2339: f64, t2019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8339 = t1540 * t511;
    let t8340 = t8339 * t650;
    let t8342 = t2011 * t1411;
    let t8343 = t8342 * t291;
    let t8344 = t2010 * t8343;
    let t8346 = t2012 * t1661;
    let t8347 = t2010 * t8346;
    let t8349 = t2020 * t2339;
    let t8350 = t2019 * t8349;
    (t8339, t8340, t8342, t8343, t8344, t8346, t8347, t8349, t8350)
}
