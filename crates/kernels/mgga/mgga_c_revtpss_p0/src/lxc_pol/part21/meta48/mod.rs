//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk356;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk357;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk358;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk359;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk360;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta48<F: Float>(t902: F, t908: F, t324: F, t320: F, t315: F, t928: F, t919: F, t924: F, t932: F, t323: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t958, t960) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk356::<F>(t902, t908);
        let (t961, t963, t964) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk357::<F>(t324, t960, t320);
        let t965 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk358::<F>(t315, t964);
        let (t967, t970, t972) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk359::<F>(t902, t928, t908, t919, t924, t932);
        let t973 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk360::<F>(t323);
        let t974 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk361::<F>(t972, t973);
    (t958, t960, t961, t963, t964, t965, t967, t970, t972, t973, t974)
}
