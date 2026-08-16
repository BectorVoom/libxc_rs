//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2138;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2139;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta511<F: Float>(t11631: F, t3151: F, t15907: F, t3117: F, t3057: F, t380: F, t3088: F, t370: F, t4757: F, t906: F, t3092: F, t994: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16082, t16083, t16084, t16087, t16088) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2138::<F>(t11631, t3151, t15907, t3117, t3057, t380, t3088, t370);
        let t16089 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2139::<F>(t16087, t16088);
        let (t16090, t16091, t16094, t16095) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2140::<F>(t4757, t906, t3092, t380, t994, t16088);
    (t16082, t16083, t16084, t16087, t16088, t16089, t16090, t16091, t16094, t16095)
}
