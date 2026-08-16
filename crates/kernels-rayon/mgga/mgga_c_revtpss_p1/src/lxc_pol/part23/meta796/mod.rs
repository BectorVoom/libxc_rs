//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta796 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2618;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta796(t10744: f64, t18409: f64, t808: f64, t18414: f64, t40521: f64, t40791: f64, t5989: f64, t10890: f64, t5985: f64, t14686: f64, t18525: f64, t50570: f64, t61956: f64, t14923: f64, t18428: f64, t10760: f64, t40627: f64, t61837: f64, t18527: f64, t50295: f64, t18353: f64, t2689: f64, t18394: f64, t2703: f64, t10777: f64, t61715: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62069, t62072, t62089, t62095, t62105) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2618(t10744, t18409, t808, t18414, t40521, t40791, t5989, t10890, t5985, t14686, t18525, t50570, t61956);
        let (t62108, t62111, t62114, t62129, t62135, t62148) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2619(t14923, t18428, t10760, t40627, t61837, t18527, t50295, t18353, t2689, t18394, t2703, t10777, t14686, t61715, t837);
    (t62069, t62072, t62089, t62095, t62105, t62108, t62111, t62114, t62129, t62135, t62148)
}
