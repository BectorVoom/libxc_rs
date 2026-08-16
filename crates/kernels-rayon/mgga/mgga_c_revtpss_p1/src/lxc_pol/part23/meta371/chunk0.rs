//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1700/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1700(t3172: f64, t4874: f64, t3127: f64, t4802: f64, t1063: f64, t4807: f64, t3153: f64, t4866: f64, t11922: f64, t4911: f64, t3115: f64, t1032: f64, t4743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15769 = t3172 * t4874;
    let t15771 = 0.19055119163586549765e-3_f64 * t3127 * t15769;
    let t15772 = t3172 * t4802;
    let t15774 = 0.3811023832717309953e-3_f64 * t1063 * t15772;
    let t15775 = t3172 * t4807;
    let t15776 = t1063 * t15775;
    let t15780 = t4866 * t3153;
    let t15794 = t11922 * t4911;
    let t15796 = 0.28582678745379824648e-3_f64 * t3115 * t15794;
    let t15816 = t4743 * t1032;
    (t15769, t15771, t15772, t15774, t15775, t15776, t15780, t15794, t15796, t15816)
}
