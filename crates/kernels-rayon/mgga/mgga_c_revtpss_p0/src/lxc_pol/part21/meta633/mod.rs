//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2402;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta633(t10910: f64, t822: f64, t10959: f64, t2439: f64, t2777: f64, t686: f64, t72: f64, t874: f64, t10914: f64, t2710: f64, t9285: f64, t10972: f64, t2470: f64, t136: f64, t2457: f64, t2760: f64, t10073: f64, t10929: f64, t10069: f64, t10654: f64, t2790: f64, t9292: f64, t2444: f64, t2829: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40927, t40938, t40942, t40945, t40948) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2402(t10910, t822, t10959, t2439, t2777, t686, t72, t874, t10914, t2710, t9285, t10972, t2470);
        let (t40952, t40954, t40956, t40958, t40968) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2403(t136, t2457, t2710, t2760, t10073, t10929, t10069, t10654, t2790, t9292, t2444, t2829, t689);
    (t40927, t40938, t40942, t40945, t40948, t40952, t40954, t40956, t40958, t40968)
}
