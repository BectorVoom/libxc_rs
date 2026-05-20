//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1391;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta437<F: Float>(t10111: F, t1386: F, t9720: F, t281: F, t39644: F, t40650: F, t547: F, t550: F, t40688: F, t2682: F, t820: F, t2735: F, t5744: F, t4086: F, t9801: F, t1320: F, t9545: F, t40082: F, t512: F, t520: F, t1333: F, t9410: F, t3853: F, t3863: F, t1340: F, t40086: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46856, t46885, t46888, t46917, t46929) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1391::<F>(t10111, t1386, t9720, t281, t39644, t40650, t547, t550, t40688, t2682, t820, t2735, t5744);
        let (t46946, t46963, t46970, t46972, t46980, t46988) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1392::<F>(t4086, t9801, t1320, t9545, t40082, t512, t520, t1333, t9410, t3853, t3863, t1340, t40086);
    (t46856, t46885, t46888, t46917, t46929, t46946, t46963, t46970, t46972, t46980, t46988)
}
