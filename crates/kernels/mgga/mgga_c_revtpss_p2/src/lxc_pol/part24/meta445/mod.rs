//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1405;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1406;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta445<F: Float>(t22: F, t46389: F, t543: F, t5735: F, t1432: F, t5763: F, t9288: F, t14202: F, t9303: F, t14238: F, t2453: F, t10139: F, t14219: F, t9285: F, t1892: F, t5744: F, t786: F, t1320: F, t13632: F, t1317: F, t3857: F, t5569: F, t1856: F, t512: F, t9544: F, t5571: F, t9387: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47967, t47971, t48005, t48007, t48036) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1405::<F>(t22, t46389, t543, t5735, t1432, t5763, t9288, t14202, t9303, t14238, t2453, t10139, t14219, t9285);
        let (t48084, t48152, t48225, t48227, t48243, t48262) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1406::<F>(t1892, t5744, t786, t1320, t13632, t1317, t3857, t5569, t1856, t512, t9544, t5571, t9387);
    (t47967, t47971, t48005, t48007, t48036, t48084, t48152, t48225, t48227, t48243, t48262)
}
