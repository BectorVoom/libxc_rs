//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2403/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2403(t136: f64, t2457: f64, t2710: f64, t2760: f64, t10073: f64, t10929: f64, t10069: f64, t10654: f64, t2790: f64, t9292: f64, t2444: f64, t2829: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t40952 = t2710 * t2760 * t136 * t2457;
    let t40954 = t10073 * t10929;
    let t40956 = t10069 * t10654;
    let t40958 = t9292 * t2790;
    let t40968 = t689 * t2444 * t2829;
    (t40952, t40954, t40956, t40958, t40968)
}
