//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1389;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta436<F: Float>(t235: F, t5744: F, t2453: F, t1389: F, t268: F, t2452: F, t40633: F, t547: F, t40634: F, t550: F, t9718: F, t247: F, t548: F, t9722: F, t1379: F, t40846: F, t816: F, t1412: F, t9794: F, t40609: F, t4062: F, t2735: F, t9792: F, t1376: F, t40769: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46802, t46810, t46817, t46820) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1389::<F>(t235, t5744, t2453, t1389, t268, t2452, t40633, t547, t40634, t550, t9718, t247, t548, t9722);
        let (t46824, t46825, t46831, t46835, t46840) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1390::<F>(t1379, t40846, t550, t816, t1412, t9794, t40609, t4062, t2735, t9792, t1376, t40769);
    (t46802, t46810, t46817, t46820, t46824, t46825, t46831, t46835, t46840)
}
