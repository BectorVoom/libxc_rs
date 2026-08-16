//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1642;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1643;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta333<F: Float>(t1042: F, t11285: F, t2866: F, t914: F, t936: F, t2869: F, t2919: F, t2923: F, t910: F, t2927: F, t287: F, t2922: F, t275: F, t2875: F, t934: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11286, t11289, t11291, t11293, t11294) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1642::<F>(t1042, t11285, t2866, t914, t936, t2869, t2919, t2923, t910);
        let (t11296, t11298, t11299) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1643::<F>(t11294, t2927, t287, t2922, t275);
        let t11300 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1644::<F>(t2875, t934);
    (t11286, t11289, t11291, t11293, t11294, t11296, t11298, t11299, t11300)
}
