//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2080;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta633(t25431: f64, t99389: f64, t1568: f64, t786: f64, t25410: f64, t25413: f64, t25375: f64, t99365: f64, t1579: f64, t25392: f64, t4481: f64, t92921: f64, t10073: f64, t1958: f64, t25390: f64, t25305: f64, t99380: f64, t213: f64, t27265: f64, t2453: f64, t2458: f64, t7760: f64, t25331: f64, t27213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99391, t99403, t99404, t99406, t99412, t99414, t99420) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2080(t25431, t99389, t1568, t786, t25410, t25413, t25375, t99365, t1579, t25392, t4481, t92921);
        let (t99423, t99425, t99429, t99435, t99456) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2081(t10073, t1579, t1958, t25390, t25305, t99380, t213, t27265, t2453, t2458, t7760, t25331, t27213);
    (t99391, t99403, t99404, t99406, t99412, t99414, t99420, t99423, t99425, t99429, t99435, t99456)
}
