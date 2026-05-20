//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta840 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2970;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta840<F: Float>(t13981: F, t9962: F, t13951: F, t2713: F, t3964: F, t1413: F, t46835: F, t48698: F, t13845: F, t13847: F, t13848: F, t4004: F, t1872: F, t9818: F, t1873: F, t46651: F, t1399: F, t5689: F, t9816: F, t3924: F) -> (F, F, F, F, F, F, F, F) {
        let (t49005, t49008, t49012, t49016) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2970::<F>(t13981, t9962, t13951, t2713, t3964, t1413, t46835, t48698, t13845, t13847, t13848, t4004);
        let (t49024, t49030, t49049, t49053) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2971::<F>(t13845, t1872, t4004, t9818, t1873, t46651, t1399, t5689, t9816, t13847, t13848, t3924);
    (t49005, t49008, t49012, t49016, t49024, t49030, t49049, t49053)
}
