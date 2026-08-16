//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1031/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1031(t481: f64, t7469: f64, t2568: f64, t3433: f64, t2563: f64, t1550: f64, t7338: f64, t2252: f64, t921: f64, t1543: f64, t2841: f64, t2567: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24454 = t7469 * t481;
    let t24521 = t3433 * t2568;
    let t24573 = t3433 * t2563;
    let t24714 = t7338 * t1550;
    let t24750 = t921 * t2252;
    let t24762 = t2841 * t1543;
    let t24786 = t2567 * t2252;
    (t24454, t24521, t24573, t24714, t24750, t24762, t24786)
}
