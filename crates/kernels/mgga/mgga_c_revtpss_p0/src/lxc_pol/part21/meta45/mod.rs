//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta45 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk338;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk339;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk340;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk341;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk342;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk343;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta45<F: Float>(t606: F, t905: F, t904: F, t128: F, t903: F, t291: F, t287: F, t275: F, t276: F, t902: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t906 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk338::<F>(t606, t905);
        let (t907, t908) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk339::<F>(t904, t906, t128);
        let t910 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk340::<F>(t903, t908);
        let (t912, t913, t914) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk341::<F>(t291, t910, t287);
        let t915 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk342::<F>(t275, t914);
        let t916 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk343::<F>(t276);
        let t918 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk344::<F>(t902, t908);
    (t906, t907, t908, t910, t912, t913, t914, t915, t916, t918)
}
