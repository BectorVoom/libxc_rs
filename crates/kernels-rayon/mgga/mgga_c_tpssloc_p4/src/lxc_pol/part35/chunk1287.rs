//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1287/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1287(t26197: f64, t80670: f64, t1834: f64, t213: f64, t225: f64, t22724: f64, t26474: f64, t22642: f64, t22643: f64, t7700: f64, t22716: f64, t7701: f64) -> (f64, f64, f64, f64, f64) {
    let t90551 = t80670 * t26197;
    let t90566 = t213 * t1834 * t225;
    let t90582 = t22724 * t26474;
    let t90642 = t22642 * t22643 * t7700;
    let t90659 = t22716 * t7701;
    (t90551, t90566, t90582, t90642, t90659)
}
