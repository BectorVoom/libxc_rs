//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1046/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1046(t113: f64, t29730: f64, t481: f64, t9272: f64, t28325: f64, t2526: f64, t2567: f64, t3056: f64, t3071: f64, t27914: f64, t10024: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29731 = t29730 * t113;
    let t29764 = t9272 * t481;
    let t29775 = t28325 * t113;
    let t29779 = t2567 * t2526;
    let t29783 = t3056 * t481;
    let t29837 = t3071 * t481;
    let t29936 = t27914 * t113;
    let t29946 = t10024 * t494;
    (t29731, t29764, t29775, t29779, t29783, t29837, t29936, t29946)
}
