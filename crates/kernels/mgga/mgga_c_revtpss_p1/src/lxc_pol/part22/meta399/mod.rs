//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1988;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1989;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta399<F: Float>(t3936: F, t5674: F, t9810: F, t125: F, t5591: F, t1399: F, t4057: F, t5704: F, t1872: F, t9818: F, t9816: F, t5706: F, t9962: F, t13944: F, t5673: F, t5675: F, t9955: F, t9956: F, t4000: F, t820: F, t844: F, t5677: F, t3934: F, t5671: F, t9847: F, t9896: F, t9906: F, t9910: F, t9919: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13967, t13977, t13981, t13985, t13987, t13988) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1988::<F>(t3936, t5674, t9810, t125, t5591, t1399, t4057, t5704, t1872, t9818, t9816, t5706, t9962);
        let (t13991, t13995, t13999) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1989::<F>(t13944, t5673, t5675, t5674, t9955, t9956, t4000, t820, t844);
        let (t14001, t14002) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1990::<F>(t13999, t5677, t13967, t13977, t13981, t13987, t13988, t13991, t13995, t3934, t5671, t9847, t9896, t9906, t9910, t9919);
    (t13967, t13977, t13981, t13985, t13987, t13988, t13991, t13995, t13999, t14001, t14002)
}
