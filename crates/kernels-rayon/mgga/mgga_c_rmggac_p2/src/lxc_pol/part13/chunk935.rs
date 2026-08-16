//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 935/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk935(t1971: f64, t2144: f64, t31125: f64, t3351: f64, t2010: f64, t8342: f64, t935: f64, t2415: f64, t4029: f64, t1240: f64, t515: f64, t570: f64, t7230: f64) -> (f64, f64, f64, f64) {
    let t40541 = t3351 * t1971 * t2144 * t31125;
    let t40544 = t2010 * t8342 * t935;
    let t40547 = t2010 * t2415 * t4029;
    let t40554 = t7230 * t1971 * t515 * t570 * t1240;
    (t40541, t40544, t40547, t40554)
}
