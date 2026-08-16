//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 970/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk970(t10115: f64, t557: f64, t10024: f64, t268: f64, t543: f64, t4101: f64, t1429: f64, t9292: f64, t3964: f64, t4096: f64, t9285: f64, t1385: f64, t4066: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10117 = 0.11044544084478153697e-3_f64 * t10115 * t557;
    let t10119 = t268 * t10024 * t543;
    let t10120 = t4101 * t10119;
    let t10126 = 0.17073386770573548589e-1_f64 * t9292 * t1429;
    let t10129 = 0.46263278077393568556e-2_f64 * t3964 * t4096 * t9285;
    let t10130 = t1385 * t4066;
    (t10117, t10119, t10120, t10126, t10129, t10130)
}
