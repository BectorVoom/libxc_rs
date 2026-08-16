//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 617/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk617(t3434: f64, t3437: f64, t3439: f64, t1102: f64, t1104: f64, t3314: f64, t261: f64, t869: f64) -> (f64, f64, f64) {
    let t3441 = t3434 * t3437 * t3439;
    let t3444 = t1102 * t3314 * t1104;
    let t3446 = t869 * t261;
    (t3441, t3444, t3446)
}
