//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta454<F: Float>(t9744: F, t9966: F, t3889: F, t4003: F, t3855: F, t3860: F, t1320: F, t9545: F, t3863: F, t39419: F, t39422: F, t46280: F, t46282: F, t46287: F, t46290: F, t46292: F, t46297: F, t46303: F) -> (F, F, F, F, F, F) {
        let (t46949, t46951, t46961, t46963, t46965, t46966) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1737::<F>(t9744, t9966, t3889, t4003, t3855, t3860, t1320, t9545, t3863, t39419, t39422, t46280, t46282, t46287, t46290, t46292, t46297, t46303);
    (t46949, t46951, t46961, t46963, t46965, t46966)
}
