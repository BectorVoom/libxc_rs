//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2806/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2806(t10073: f64, t10654: f64, t10959: f64, t2439: f64, t2777: f64, t10914: f64, t2710: f64, t9285: f64, t10972: f64, t2470: f64, t874: f64, t136: f64, t2457: f64, t2760: f64) -> (f64, f64, f64, f64, f64) {
    let t40924 = t10073 * t10654;
    let t40938 = t2439 * t2777 * t10959;
    let t40945 = t2710 * t10914 * t9285;
    let t40948 = t874 * t10972 * t2470;
    let t40952 = t2710 * t2760 * t136 * t2457;
    (t40924, t40938, t40945, t40948, t40952)
}
