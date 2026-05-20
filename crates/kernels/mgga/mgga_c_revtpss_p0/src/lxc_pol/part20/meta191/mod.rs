//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk949;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk950;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta191<F: Float>(t543: F, t9898: F, t1390: F, t828: F, t221: F, t4019: F, t4057: F, t4018: F, t1386: F, t2681: F, t820: F, t1401: F, t4003: F, t4000: F, t843: F, t4006: F, t136: F, t4011: F, t3829: F, t3978: F, t3970: F, t3989: F, t1388: F, t3934: F, t4002: F, t5671: F, t9828: F, t9832: F, t9837: F, t9842: F, t9847: F, t9893: F, t9896: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9899, t9901, t9905, t9906, t9909, t9910) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk949::<F>(t543, t9898, t1390, t828, t221, t4019, t4057, t4018, t1386, t2681, t820, t1401);
        let (t9912, t9914, t9918, t9919, t9921, t9923) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk950::<F>(t4003, t9898, t1390, t828, t4000, t820, t843, t4006, t136, t4011, t221, t3829);
        let t9928 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk951::<F>(t3978, t9923, t3970, t3989, t1388, t3934, t4002, t5671, t9828, t9832, t9837, t9842, t9847, t9893, t9896, t9901, t9906, t9910, t9914, t9919);
    (t9899, t9901, t9905, t9909, t9912, t9914, t9918, t9921, t9923, t9928)
}
