//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 957/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk957(t1102: f64, t3314: f64, t3457: f64, t2333: f64, t481: f64, t795: f64, t2304: f64, t875: f64, t3434: f64, t3439: f64, t106: f64, t1550: f64, t97: f64) -> (f64, f64, f64, f64, f64) {
    let t11008 = t1102 * t3314 * t3457;
    let t11010 = t2333 * t481;
    let t11011 = t11010 * t795;
    let t11015 = t2304 * t875;
    let t11017 = t3434 * t11015 * t3439;
    let t11020 = t97 * t106 * t1550;
    (t11008, t11011, t11015, t11017, t11020)
}
