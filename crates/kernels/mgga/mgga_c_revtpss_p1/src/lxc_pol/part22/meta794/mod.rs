//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta794 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2889;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta794<F: Float>(t9761: F, t9765: F, t240: F, t9991: F, t3995: F, t40488: F, t549: F, t72: F, t4014: F, t9779: F, t1408: F, t2237: F, t2482: F, t3981: F, t1412: F, t3889: F, t808: F, t9736: F, t1369: F, t9726: F, t1372: F, t546: F, t9801: F, t9738: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46602, t46609, t46620, t46627, t46633, t46644) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2889::<F>(t9761, t9765, t240, t9991, t3995, t40488, t549, t72, t4014, t9779, t1408, t2237, t2482);
        let (t46645, t46649, t46651, t46652, t46670, t46671) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2890::<F>(t3981, t46644, t1412, t3889, t808, t9736, t1369, t9726, t1372, t546, t9801, t9738);
    (t46602, t46609, t46620, t46627, t46633, t46644, t46645, t46649, t46651, t46652, t46670, t46671)
}
