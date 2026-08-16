//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 856/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk856(t157: f64, t406: f64, t864: f64, t1487: f64, t435: f64, t361: f64, t171: f64, t3300: f64, t6576: f64, t814: f64, t1454: f64, t322: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15758 = t864 * t406 * t157;
    let t15995 = t435 * t1487;
    let t17752 = t361 * t435;
    let t17912 = t171 * t3300;
    let t19418 = t6576 * t814;
    let t20311 = t1454 * t322;
    (t15758, t15995, t17752, t17912, t19418, t20311)
}
