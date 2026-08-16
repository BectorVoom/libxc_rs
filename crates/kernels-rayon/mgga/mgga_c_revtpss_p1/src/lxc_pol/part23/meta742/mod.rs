//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta742 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2522;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta742(t2718: f64, t4469: f64, t4519: f64, t9292: f64, t2798: f64, t4499: f64, t9288: f64, t2783: f64, t786: f64, t10073: f64, t14588: f64, t10542: f64, t14563: f64, t14519: f64, t2470: f64, t4522: f64, t874: f64, t1573: f64, t40317: f64, t10069: f64, t14496: f64, t14524: f64, t39575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51396, t51403, t51408, t51421, t51424, t51429) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2522(t2718, t4469, t4519, t9292, t2798, t4499, t9288, t2783, t786, t10073, t14588, t10542, t14563);
        let (t51430, t51435, t51445, t51452, t51471, t51483) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2523(t51429, t14519, t2470, t2798, t4522, t874, t9288, t1573, t40317, t10069, t14496, t14524, t39575);
    (t51396, t51403, t51408, t51421, t51424, t51430, t51435, t51445, t51452, t51471, t51483)
}
