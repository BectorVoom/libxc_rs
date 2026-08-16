//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1014/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1014(t322: f64, t11893: f64, t3633: f64, t833: f64, t3638: f64, t829: f64, t1013: f64, t3370: f64, t1074: f64, t2394: f64, t11063: f64, t11066: f64, t1300: f64, t2400: f64, t327: f64, t3373: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t11894 = piecewise3(t324, 0.0_f64, t11893);
    let t11897 = t3633 * t833;
    let t11906 = t3638 * t829;
    let t11909 = t3370 * t1013;
    let t11912 = t1074 * t2394;
    let t11915 = t3633 * t829;
    let t11920 = -0.64e0_f64 * t11894 * t327 - 0.128e1_f64 * t11897 * t829 - 0.128e1_f64 * t11063 * t1013 - 0.384e1_f64 * t11066 * t2400 - 0.128e1_f64 * t3373 * t2394 - 0.384e1_f64 * t6693 * t11906 - 0.128e1_f64 * t1300 * t11909 - 0.128e1_f64 * t1300 * t11912 - 0.128e1_f64 * t1300 * t11915 - 0.64e0_f64 * t834 * t11894;
    (t11894, t11897, t11909, t11912, t11920)
}
