//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta824 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3066;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3067;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3068;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3069;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3070;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3071;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta824<F: Float>(t16716: F, t689: F, t12256: F, t2251: F, t4186: F, t12305: F, t128: F, t13312: F, t3367: F, t606: F, t1120: F, t2435: F, t5057: F, t16747: F, t1121: F, t49889: F, t1716: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t56216 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3066::<F>(t16716, t689);
        let (t56219, t56221) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3067::<F>(t12256, t2251, t4186, t12305, t128);
        let (t56224, t56226) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3068::<F>(t13312, t3367, t606, t1120, t128);
        let t56228 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3069::<F>(t2435, t5057);
        let (t56229, t56230) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3070::<F>(t56228, t16747, t689);
        let (t56232, t56234) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3071::<F>(t1121, t49889, t1120, t128);
        let t56236 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3072::<F>(t1716, t9292);
    (t56216, t56219, t56221, t56224, t56226, t56228, t56229, t56230, t56232, t56234, t56236)
}
