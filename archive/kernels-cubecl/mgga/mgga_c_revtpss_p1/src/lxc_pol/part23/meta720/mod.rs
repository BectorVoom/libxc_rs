//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2480;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta720<F: Float>(t48614: F, t14005: F, t46740: F, t46917: F, t5697: F, t14036: F, t9976: F, t46694: F, t5686: F, t13769: F, t808: F, t9736: F, t13952: F, t2689: F, t13784: F, t543: F, t46825: F, t9793: F, t1353: F, t1883: F, t1408: F, t241: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48615, t48638, t48645, t48669, t48686, t48690) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2480::<F>(t48614, t14005, t46740, t46917, t5697, t14036, t9976, t46694, t5686, t13769, t808, t9736);
        let (t48691, t48692, t48694, t48696, t48698, t48700, t48712) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2481::<F>(t48690, t13952, t2689, t13784, t543, t46825, t9793, t1353, t1883, t1408, t241, t820);
    (t48615, t48638, t48645, t48669, t48686, t48691, t48692, t48694, t48696, t48698, t48700, t48712)
}
