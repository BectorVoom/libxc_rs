//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk345;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk346;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk347;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta46<F: Float>(t916: F, t918: F, t902: F, t273: F, t240: F, t696: F, t281: F, t283: F, t346: F, t906: F, t141: F, t908: F, t290: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t919, t921, t923) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk345::<F>(t916, t918, t902, t273);
        let (t924, t926, t928, t929, t930) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk346::<F>(t918, t923, t240, t696, t281, t283, t346);
        let (t931, t932, t934) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk347::<F>(t906, t930, t141, t908, t919, t921, t924, t929);
        let t935 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk348::<F>(t290);
    (t919, t921, t923, t924, t926, t928, t929, t930, t931, t932, t934, t935)
}
