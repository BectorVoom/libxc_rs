//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta727 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2494;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta727<F: Float>(t49321: F, t1897: F, t40317: F, t10111: F, t22: F, t5759: F, t14188: F, t2439: F, t2777: F, t10073: F, t14129: F, t14159: F, t3964: F, t9285: F, t213: F, t225: F, t46475: F, t5600: F, t9292: F, t1893: F, t4075: F, t786: F, t10115: F, t1894: F, t14094: F, t2435: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49322, t49354, t49361, t49426, t49429, t49432) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2494::<F>(t49321, t1897, t40317, t10111, t22, t5759, t14188, t2439, t2777, t10073, t14129, t14159, t3964, t9285);
        let (t49439, t49468, t49471, t49474, t49476) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2495::<F>(t213, t225, t46475, t5600, t9292, t1893, t4075, t786, t10115, t1894, t14094, t2435);
    (t49322, t49354, t49361, t49426, t49429, t49432, t49439, t49468, t49471, t49474, t49476)
}
