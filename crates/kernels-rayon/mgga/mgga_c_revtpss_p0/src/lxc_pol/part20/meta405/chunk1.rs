//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1499/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1499(t1063: f64, t247: f64, t2853: f64, t42447: f64, t11151: f64, t11725: f64, t1042: f64, t11653: f64, t11714: f64, t11748: f64, t15716: f64, t15728: f64, t15935: f64, t3101: f64, t3116: f64, t3127: f64, t3130: f64, t3182: f64, t3188: f64, t41277: f64, t42001: f64, t42421: f64, t42425: f64, t42428: f64, t42439: f64) -> f64 {
    let t42450 = t1063 * t247 * t42447 * t2853;
    let t42454 = t1063 * t247 * t11725 * t11151;
    let t42456 = -0.1219527626469539185e-1_f64 * t42421 - 0.18292914397043087774e-1_f64 * t15728 * t11653 - 0.57927562257303111285e-1_f64 * t42425 * t3130 - 0.34299214494455789577e-2_f64 * t3127 * t1042 * t15935 * t42428 + 0.85748036236139473944e-2_f64 * t1063 * t247 * t3182 * t41277 + 0.18292914397043087774e-1_f64 * t11714 * t3101 - 0.22866142996303859718e-2_f64 * t42439 + 0.34299214494455789577e-2_f64 * t3188 * t11748 - 0.77173232612525526552e-2_f64 * t15716 * t247 * t3116 * t42001 - 0.31758531939310916276e-3_f64 * t42450 - 0.3811023832717309953e-2_f64 * t42454;
    t42456
}
