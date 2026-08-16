//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 823/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk823(t10115: f64, t557: f64, t1429: f64, t9292: f64, t3964: f64, t4096: f64, t9285: f64, t2453: f64, t4100: f64, t562: f64, t64: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10117 = 0.11044544084478153697e-3_f64 * t10115 * t557;
    let t10126 = 0.17073386770573548589e-1_f64 * t9292 * t1429;
    let t10129 = 0.46263278077393568556e-2_f64 * t3964 * t4096 * t9285;
    let t10139 = t2453 * t4100;
    let t10157 = 0.11044544084478153697e-3_f64 * t10115 * t562;
    let t10199 = t64 * t843;
    (t10117, t10126, t10129, t10139, t10157, t10199)
}
