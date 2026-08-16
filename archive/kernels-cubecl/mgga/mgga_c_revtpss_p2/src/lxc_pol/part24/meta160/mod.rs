//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta160 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk802;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk803;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk804;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk805;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk806;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta160<F: Float>(t3360: F, t6421: F, t128: F, t3367: F, t5819: F, t1120: F, t1121: F, t5825: F, t3357: F, t5044: F, t422: F, t1733: F, t5063: F, t1732: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6422, t6423) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk802::<F>(t3360, t6421, t128);
        let t6425 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk803::<F>(t3367, t5819);
        let (t6426, t6427) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk804::<F>(t1120, t6425, t128);
        let t6429 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk805::<F>(t1121, t5825);
        let (t6430, t6431) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk806::<F>(t1120, t6429, t128);
        let (t6433, t6435, t6437, t6438) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk807::<F>(t3357, t5044, t6423, t6427, t6431, t422, t1733, t5063, t1732);
    (t6422, t6423, t6425, t6426, t6427, t6429, t6430, t6431, t6433, t6435, t6437, t6438)
}
