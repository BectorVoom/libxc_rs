//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta875 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2778;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta875<F: Float>(t2661: F, t3992: F, t48533: F, t6869: F, t14045: F, t22096: F, t21990: F, t5608: F, t9934: F, t1413: F, t46835: F, t74483: F, t22061: F, t9793: F, t9794: F, t22093: F, t9962: F, t13845: F, t73731: F, t9818: F, t9835: F, t13847: F, t13848: F) -> (F, F, F, F, F, F, F, F) {
        let (t74598, t74602, t74606, t74638) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2778::<F>(t2661, t3992, t48533, t6869, t14045, t22096, t21990, t5608, t9934, t1413, t46835, t74483);
        let (t74641, t74656, t74660, t74664) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2779::<F>(t22061, t9793, t9794, t22093, t9962, t13845, t73731, t9818, t9835, t13847, t13848, t21990);
    (t74598, t74602, t74606, t74638, t74641, t74656, t74660, t74664)
}
