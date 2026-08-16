//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta253(t11315: f64, t923: f64, t11156: f64, t2908: f64, t141: f64, t11165: f64, t930: f64, t2912: f64, t698: f64, t11151: f64, t11160: f64, t11132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11316, t11318, t11319, t11321, t11322, t11326, t11328, t11329, t11331, t11332, t11334) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1086(t11315, t923, t11156, t2908, t141, t11165, t930, t2912, t698, t11151, t11160, t11132);
    (t11316, t11318, t11319, t11321, t11322, t11326, t11328, t11329, t11331, t11332, t11334)
}
