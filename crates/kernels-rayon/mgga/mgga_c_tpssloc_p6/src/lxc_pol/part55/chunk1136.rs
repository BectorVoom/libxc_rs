//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1136/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1136(t225: f64, t25222: f64, t25220: f64, t28: f64, t40772: f64, t1834: f64, t794: f64, t213: f64, t26219: f64, t214: f64, t5318: f64, t1824: f64, t6955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87810 = t25222 * t225;
    let t87837 = t25220 * t225;
    let t89953 = t40772 * t28;
    let t90544 = t794 * t1834;
    let t90566 = t213 * t1834 * t225;
    let t90732 = t26219 * t225;
    let t90739 = t214 * t5318;
    let t90942 = t6955 * t1824;
    (t87810, t87837, t89953, t90544, t90566, t90732, t90739, t90942)
}
