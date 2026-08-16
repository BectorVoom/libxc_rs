//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 810/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk810(t674: f64, t7715: f64, t8601: f64, t1997: f64, t8607: f64, t7696: f64, t9222: f64, t35589: f64, t570: f64, t739: f64, t7255: f64, t9171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38370 = t8601 * t7715 * t674;
    let t38371 = t38370 * t1997;
    let t38374 = t8607 * t7715 * t674;
    let t38375 = t38374 * t1997;
    let t38377 = t9222 * t7696;
    let t38381 = t35589 * t570;
    let t38382 = t739 * t38381;
    let t38387 = t7255 * t9171;
    (t38371, t38375, t38377, t38381, t38382, t38387)
}
