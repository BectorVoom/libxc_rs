//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1310;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1311;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1312;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta219<F: Float>(t1214: F, t1469: F, t5296: F, t1042: F, t3362: F, t3617: F, t4181: F, t1012: F, t1224: F, t5052: F, t3698: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5297, t5298, t5299, t5302) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1310::<F>(t1214, t1469, t5296, t1042, t3362, t3617);
        let (t5303, t5304, t5308) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1311::<F>(t4181, t5302, t1042, t1012, t1224);
        let (t5309, t5312) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1312::<F>(t5052, t5308, t1012, t3698);
    (t5297, t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312)
}
