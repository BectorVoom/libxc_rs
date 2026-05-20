//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk327;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk328;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta48<F: Float>(t421: F, t1118: F, t431: F, t426: F, t1143: F, t434: F, t444: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t1150 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk327::<F>(t421);
        let (t1154, t1159, t1160, t1161, t1163, t1166, t1169) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk328::<F>(t1118, t431, t426, t1143, t434);
        let (t1173, t1178, t1179) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk329::<F>(t1118, t444);
    (t1150, t1154, t1159, t1160, t1161, t1163, t1166, t1169, t1173, t1178, t1179)
}
