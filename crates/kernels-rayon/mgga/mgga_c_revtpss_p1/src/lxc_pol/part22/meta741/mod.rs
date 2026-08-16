//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2806;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta741(t10073: f64, t10654: f64, t10959: f64, t2439: f64, t2777: f64, t10914: f64, t2710: f64, t9285: f64, t10972: f64, t2470: f64, t874: f64, t136: f64, t2457: f64, t2760: f64, t10929: f64, t10069: f64, t2790: f64, t9292: f64, t11003: f64, t9303: f64, t10981: f64, t22: f64, t868: f64, t886: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40924, t40938, t40945, t40948, t40952) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2806(t10073, t10654, t10959, t2439, t2777, t10914, t2710, t9285, t10972, t2470, t874, t136, t2457, t2760);
        let (t40954, t40956, t40958, t40970, t40978) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2807(t10073, t10929, t10069, t10654, t2790, t9292, t11003, t9303, t10981, t22, t868, t886);
    (t40924, t40938, t40945, t40948, t40952, t40954, t40956, t40958, t40970, t40978)
}
