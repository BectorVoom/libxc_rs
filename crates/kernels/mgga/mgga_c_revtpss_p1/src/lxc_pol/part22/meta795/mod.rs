//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta795 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2891;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta795<F: Float>(t794: F, t9747: F, t2699: F, t3943: F, t3946: F, t3995: F, t40690: F, t9775: F, t9936: F, t3970: F, t9779: F, t9765: F, t9923: F, t136: F, t9941: F, t1386: F, t820: F, t9948: F, t1401: F, t159: F, t216: F, t4010: F, t2482: F, t2668: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46691, t46694, t46695, t46702, t46704, t46706, t46712) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2891::<F>(t794, t9747, t2699, t3943, t3946, t3995, t40690, t9775, t9936, t3970, t9779, t9765, t9923);
        let (t46716, t46722, t46723, t46730, t46740) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2892::<F>(t136, t9941, t1386, t820, t9948, t1401, t159, t216, t4010, t2482, t2668);
    (t46691, t46694, t46695, t46702, t46704, t46706, t46712, t46716, t46722, t46723, t46730, t46740)
}
