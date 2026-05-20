//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta910 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2923;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2924;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta910<F: Float>(t52091: F, t52092: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F, t77778: F, t923: F, t52035: F, t52037: F, t77539: F, t77543: F, t77547: F, t916: F, t23510: F, t698: F, t23507: F, t141: F, t77533: F, t930: F, t77537: F, t77541: F, t77545: F, t52127: F, t52128: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t77797 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2923::<F>(t52091, t52092, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let (t77798, t77799, t77801) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2924::<F>(t77778, t77797, t923, t52035, t52037, t63338, t63340, t63342, t63361, t63371, t77539, t77543, t77547);
        let (t77802, t77804, t77806, t77810, t77813, t77816, t77819, t77824) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2925::<F>(t77798, t916, t23510, t698, t23507, t141, t77533, t930, t77537, t77541, t77545, t52127, t52128, t63447, t63453, t63459);
    (t77799, t77801, t77802, t77804, t77806, t77810, t77813, t77816, t77819, t77824)
}
