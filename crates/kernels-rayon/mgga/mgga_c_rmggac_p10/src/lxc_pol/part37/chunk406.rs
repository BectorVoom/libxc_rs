//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 406/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk406(t1411: f64, t2011: f64, t291: f64, t1661: f64, t2012: f64, t1665: f64, t2415: f64, t935: f64, t938: f64, t623: f64, t880: f64, t2144: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8342 = t2011 * t1411;
    let t8343 = t8342 * t291;
    let t8346 = t2012 * t1661;
    let t8352 = t2012 * t1665;
    let t8358 = t2415 * t935;
    let t8362 = t2415 * t938;
    let t8365 = t623 * t880;
    let t8368 = t623 * t2144;
    (t8342, t8343, t8346, t8352, t8358, t8362, t8365, t8368)
}
