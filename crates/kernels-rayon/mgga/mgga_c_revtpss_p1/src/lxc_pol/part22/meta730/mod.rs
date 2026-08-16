//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta730 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta730(t820: f64, t823: f64, t9948: f64, t839: f64, t10841: f64, t10845: f64, t10815: f64, t2648: f64, t2756: f64, t2681: f64, t2719: f64, t2726: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t40360, t40361, t40374, t40393, t40395, t40398, t40399) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2787(t820, t823, t9948, t839, t10841, t10845, t10815, t2648, t2756, t2681, t2719, t2726);
    (t40360, t40361, t40374, t40393, t40395, t40398, t40399)
}
