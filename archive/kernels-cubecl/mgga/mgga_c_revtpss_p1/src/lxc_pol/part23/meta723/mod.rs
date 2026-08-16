//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2486;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta723<F: Float>(t1389: F, t14230: F, t2735: F, t46801: F, t40763: F, t5609: F, t9793: F, t13830: F, t9775: F, t13760: F, t9765: F, t268: F, t5617: F, t46784: F, t124: F, t5658: F, t1889: F, t46595: F, t13850: F, t2482: F, t2668: F, t4000: F, t4010: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48877, t48879, t48881, t48905, t48908) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2486::<F>(t1389, t14230, t2735, t46801, t40763, t5609, t9793, t13830, t9775, t13760, t9765, t268, t5617);
        let (t48909, t48919, t48947, t48982, t48999) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2487::<F>(t46784, t48908, t124, t5658, t1889, t46595, t13850, t2482, t2668, t4000, t4010, t808);
    (t48877, t48879, t48881, t48905, t48908, t48909, t48919, t48947, t48982, t48999)
}
