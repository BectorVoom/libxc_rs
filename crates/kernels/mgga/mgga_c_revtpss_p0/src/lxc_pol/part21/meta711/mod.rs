//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2542;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta711<F: Float>(t3981: F, t46644: F, t1412: F, t3889: F, t808: F, t9736: F, t1369: F, t9726: F, t1372: F, t13999: F, t9837: F, t546: F, t9801: F, t9738: F, t124: F, t3938: F, t4056: F, t9816: F, t9818: F, t794: F, t9747: F, t9750: F, t2699: F, t3943: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46645, t46649, t46651, t46652, t46660, t46670) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2542::<F>(t3981, t46644, t1412, t3889, t808, t9736, t1369, t9726, t1372, t13999, t9837, t546, t9801);
        let (t46671, t46680, t46691, t46692, t46694) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2543::<F>(t46670, t9738, t124, t3938, t4056, t9816, t9818, t794, t9747, t9750, t2699, t3943);
    (t46645, t46649, t46651, t46652, t46660, t46670, t46671, t46680, t46691, t46692, t46694)
}
