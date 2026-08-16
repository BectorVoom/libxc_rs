//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 642/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk642(t322: f64, t1013: f64, t1074: f64, t1300: f64, t327: f64, t3373: f64, t3633: f64, t834: f64, t330: f64, t1018: f64, t1079: f64, t3632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t3638 = t1074 * t1013;
    let t3643 = -0.64e0_f64 * t3633 * t327 - 0.128e1_f64 * t3373 * t1013 - 0.128e1_f64 * t1300 * t3638 - 0.64e0_f64 * t834 * t3633;
    let t3644 = t3643 * t330;
    let t3645 = t1079 * t1018;
    let t3646 = t3645 * t330;
    let t3648 = piecewise3(t332, 0.0_f64, t3632);
    (t3638, t3643, t3644, t3645, t3646, t3648)
}
