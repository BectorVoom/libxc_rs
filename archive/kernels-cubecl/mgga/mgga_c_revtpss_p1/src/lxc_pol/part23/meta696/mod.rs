//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta696 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2444;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta696<F: Float>(t1390: F, t1399: F, t46856: F, t685: F, t3952: F, t9784: F, t281: F, t39644: F, t40650: F, t547: F, t550: F, t40688: F, t46786: F, t1386: F, t2682: F, t820: F, t2735: F, t5744: F, t4086: F, t9801: F, t9846: F, t1320: F, t9545: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46859, t46879, t46885, t46888) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2444::<F>(t1390, t1399, t46856, t685, t3952, t9784, t281, t39644, t40650, t547, t550, t40688);
        let (t46889, t46917, t46929, t46946, t46947, t46963) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2445::<F>(t46786, t46888, t1386, t2682, t820, t2735, t5744, t4086, t9801, t9846, t1320, t9545);
    (t46859, t46879, t46885, t46888, t46889, t46917, t46929, t46946, t46947, t46963)
}
