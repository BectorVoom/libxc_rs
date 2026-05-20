//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1410;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta448<F: Float>(t40690: F, t5610: F, t5618: F, t9784: F, t46644: F, t5622: F, t40488: F, t40763: F, t5609: F, t9793: F, t268: F, t5617: F, t46784: F, t1889: F, t46595: F, t1873: F, t46651: F, t13800: F, t46670: F, t3964: F, t9732: F, t46888: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48829, t48833, t48849, t48853, t48879, t48908) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1410::<F>(t40690, t5610, t5618, t9784, t46644, t5622, t40488, t40763, t5609, t9793, t268, t5617);
        let (t48909, t48947, t49030, t49087, t49090, t49105) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1411::<F>(t46784, t48908, t1889, t46595, t1873, t46651, t13800, t46670, t3964, t5617, t9732, t46888);
    (t48829, t48833, t48849, t48853, t48879, t48909, t48947, t49030, t49087, t49090, t49105)
}
