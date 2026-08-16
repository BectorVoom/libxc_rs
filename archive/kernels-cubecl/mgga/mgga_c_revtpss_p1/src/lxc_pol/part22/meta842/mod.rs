//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta842 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2974;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta842<F: Float>(t14230: F, t46802: F, t49068: F, t46888: F, t48908: F, t1398: F, t5591: F, t13946: F, t9962: F, t1413: F, t46835: F, t48694: F, t13775: F, t9793: F, t9794: F, t5690: F, t9741: F, t14016: F, t46691: F, t14020: F, t3957: F, t2659: F, t5744: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49103, t49105, t49107, t49118, t49121) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2974::<F>(t14230, t46802, t49068, t46888, t48908, t1398, t5591, t13946, t9962, t1413, t46835, t48694);
        let (t49124, t49126, t49128, t49134, t49137) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2975::<F>(t13775, t9793, t9794, t5690, t9741, t14016, t46691, t14020, t3957, t2659, t5744, t816);
    (t49103, t49105, t49107, t49118, t49121, t49124, t49126, t49128, t49134, t49137)
}
