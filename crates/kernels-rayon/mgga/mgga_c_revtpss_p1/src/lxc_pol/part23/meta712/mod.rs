//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2470;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta712(t14238: f64, t2453: f64, t10142: f64, t10073: f64, t14231: f64, t10139: f64, t14219: f64, t9285: f64, t14215: f64, t2470: f64, t4101: f64, t14220: f64, t46495: f64, t4086: f64, t5710: f64, t786: f64, t10014: f64, t14242: f64, t14225: f64, t1892: f64, t5744: f64, t136: f64, t2457: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48007, t48009, t48029, t48036, t48040, t48041) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2470(t14238, t2453, t10142, t10073, t14231, t10139, t14219, t9285, t14215, t2470, t4101, t14220, t46495);
        let (t48042, t48048, t48080, t48082, t48083, t48084, t48089) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2471(t48041, t4086, t5710, t786, t10014, t14242, t10073, t14225, t1892, t5744, t136, t2457, t3964);
    (t48007, t48009, t48029, t48036, t48040, t48042, t48048, t48080, t48082, t48083, t48084, t48089)
}
