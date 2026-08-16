//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2438;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta693<F: Float>(t549: F, t240: F, t72: F, t1408: F, t2237: F, t2482: F, t3981: F, t1369: F, t9726: F, t1372: F, t546: F, t9801: F, t9738: F, t794: F, t9747: F, t2699: F, t3943: F, t3995: F, t40690: F, t136: F, t9941: F, t1386: F, t820: F, t9948: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46627, t46644, t46645, t46651, t46652, t46670) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2438::<F>(t549, t240, t72, t1408, t2237, t2482, t3981, t1369, t9726, t1372, t546, t9801);
        let (t46671, t46691, t46694, t46702, t46716, t46722) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2439::<F>(t46670, t9738, t794, t9747, t2699, t3943, t3995, t40690, t136, t9941, t1386, t820, t9948);
    (t46627, t46644, t46645, t46651, t46652, t46670, t46671, t46691, t46694, t46702, t46716, t46722)
}
