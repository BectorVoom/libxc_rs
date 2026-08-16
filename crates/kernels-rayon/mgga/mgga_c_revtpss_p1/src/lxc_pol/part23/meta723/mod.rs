//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta723 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2486;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta723(t1389: f64, t14230: f64, t2735: f64, t46801: f64, t40763: f64, t5609: f64, t9793: f64, t13830: f64, t9775: f64, t13760: f64, t9765: f64, t268: f64, t5617: f64, t46784: f64, t124: f64, t5658: f64, t1889: f64, t46595: f64, t13850: f64, t2482: f64, t2668: f64, t4000: f64, t4010: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48877, t48879, t48881, t48905, t48908) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2486(t1389, t14230, t2735, t46801, t40763, t5609, t9793, t13830, t9775, t13760, t9765, t268, t5617);
        let (t48909, t48919, t48947, t48982, t48999) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2487(t46784, t48908, t124, t5658, t1889, t46595, t13850, t2482, t2668, t4000, t4010, t808);
    (t48877, t48879, t48881, t48905, t48908, t48909, t48919, t48947, t48982, t48999)
}
