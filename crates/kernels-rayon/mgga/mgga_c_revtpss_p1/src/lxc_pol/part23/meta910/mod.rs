//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta910 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2923;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2924;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta910(t52091: f64, t52092: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t63447: f64, t63453: f64, t63459: f64, t63464: f64, t77559: f64, t77561: f64, t77566: f64, t77570: f64, t77575: f64, t77581: f64, t77586: f64, t77590: f64, t77594: f64, t77778: f64, t923: f64, t52035: f64, t52037: f64, t77539: f64, t77543: f64, t77547: f64, t916: f64, t23510: f64, t698: f64, t23507: f64, t141: f64, t77533: f64, t930: f64, t77537: f64, t77541: f64, t77545: f64, t52127: f64, t52128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t77797 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2923(t52091, t52092, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let (t77798, t77799, t77801) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2924(t77778, t77797, t923, t52035, t52037, t63338, t63340, t63342, t63361, t63371, t77539, t77543, t77547);
        let (t77802, t77804, t77806, t77810, t77813, t77816, t77819, t77824) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2925(t77798, t916, t23510, t698, t23507, t141, t77533, t930, t77537, t77541, t77545, t52127, t52128, t63447, t63453, t63459);
    (t77799, t77801, t77802, t77804, t77806, t77810, t77813, t77816, t77819, t77824)
}
