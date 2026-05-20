//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta709 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2538;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta709<F: Float>(t2782: F, t4086: F, t46394: F, t543: F, t3829: F, t4010: F, t808: F, t9736: F, t1408: F, t820: F, t9948: F, t1416: F, t9775: F, t9931: F, t3989: F, t9757: F, t9761: F, t9765: F, t1353: F, t13767: F, t2661: F, t3889: F, t240: F, t9991: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t46587, t46592, t46595, t46596) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2538::<F>(t2782, t4086, t46394, t543, t3829, t4010, t808, t9736, t1408, t820, t9948, t1416);
        let (t46598, t46600, t46602, t46607, t46609) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2539::<F>(t9775, t9931, t3989, t9757, t9761, t9765, t1353, t13767, t2661, t3889, t4010, t240, t9991);
    (t46587, t46592, t46595, t46596, t46598, t46600, t46602, t46607, t46609)
}
