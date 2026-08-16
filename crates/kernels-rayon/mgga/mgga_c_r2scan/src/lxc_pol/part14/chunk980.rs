//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 980/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk980(t11470: f64, t354: f64, t2867: f64, t481: f64, t3574: f64, t792: f64, t2333: f64, t910: f64, t795: f64, t105: f64, t920: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11471 = t354 * t11470;
    let t11475 = t2867 * t481;
    let t11486 = t3574 * t792;
    let t11496 = t2333 * t910;
    let t11497 = t11496 * t795;
    let t11505 = t105 * t920;
    let t11506 = t97 * t11505;
    (t11471, t11475, t11486, t11496, t11497, t11505, t11506)
}
