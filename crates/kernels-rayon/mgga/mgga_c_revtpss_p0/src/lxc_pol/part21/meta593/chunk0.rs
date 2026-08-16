//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2310/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2310(t159: f64, t2698: f64, t1014: f64, t65: f64, t3252: f64, t1513: f64, t665: f64, t1224: f64, t3698: f64, t10208: f64, t69: f64, t1504: f64, t658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25273 = t2698 * t159;
    let t27527 = t65 * t1014;
    let t27531 = t65 * t3252;
    let t28036 = t1513 * t665;
    let t29048 = t65 * t1224;
    let t29054 = t65 * t3698;
    let t31035 = t69 * t10208;
    let t31283 = t1504 * t658;
    (t25273, t27527, t27531, t28036, t29048, t29054, t31035, t31283)
}
