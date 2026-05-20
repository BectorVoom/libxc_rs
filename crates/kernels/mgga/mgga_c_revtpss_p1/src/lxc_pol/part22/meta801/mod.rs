//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta801 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta801<F: Float>(t4086: F, t9801: F, t9846: F, t3889: F, t4003: F, t3855: F, t3860: F, t1320: F, t9545: F, t3863: F, t3857: F, t40082: F, t512: F, t520: F) -> (F, F, F, F, F, F, F, F) {
        let (t46946, t46947, t46951, t46960, t46963, t46964, t46967, t46970) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2902::<F>(t4086, t9801, t9846, t3889, t4003, t3855, t3860, t1320, t9545, t3863, t3857, t40082, t512, t520);
    (t46946, t46947, t46951, t46960, t46963, t46964, t46967, t46970)
}
