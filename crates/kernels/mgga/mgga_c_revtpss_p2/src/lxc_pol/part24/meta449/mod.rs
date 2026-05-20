//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1412;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1413;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta449<F: Float>(t5760: F, t9292: F, t40921: F, t5737: F, t4101: F, t5740: F, t9288: F, t40270: F, t1892: F, t9990: F, t1897: F, t40317: F, t10111: F, t22: F, t5759: F, t14159: F, t3964: F, t9285: F, t5600: F, t1893: F, t4075: F, t786: F, t10115: F, t1894: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49172, t49178, t49203, t49210, t49327, t49354) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1412::<F>(t5760, t9292, t40921, t5737, t4101, t5740, t9288, t40270, t1892, t9990, t1897, t40317);
        let (t49361, t49432, t49468, t49471, t49474) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1413::<F>(t10111, t22, t5759, t14159, t3964, t9285, t5600, t9292, t1893, t4075, t786, t10115, t1894);
    (t49172, t49178, t49203, t49210, t49327, t49354, t49361, t49432, t49468, t49471, t49474)
}
