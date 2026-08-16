//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta574(t17807: f64, t489: f64, t3759: f64, t5230: f64, t1811: f64, t3601: f64, t3769: f64, t16695: f64, t17454: f64, t473: f64, t5412: f64, t1214: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17808, t17811, t17814, t17815, t17818, t17821, t17822) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2281(t17807, t489, t3759, t5230, t1811, t3601, t3769, t16695, t17454, t473, t5412, t1214);
    (t17808, t17811, t17814, t17815, t17818, t17821, t17822)
}
