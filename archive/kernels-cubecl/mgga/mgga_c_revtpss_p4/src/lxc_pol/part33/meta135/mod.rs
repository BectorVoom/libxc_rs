//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk730;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk731;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta135<F: Float>(t3478: F, t3356: F, t1175: F, t1179: F, t1178: F, t444: F, t439: F, t3413: F, t447: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3479, t3483, t3491, t3495) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk730::<F>(t3478, t3356, t1175, t1179, t1178, t444);
        let (t3496, t3503, t3510, t3519, t3520) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk731::<F>(t3495, t439, t3356, t3413, t1178);
        let (t3521, t3522, t3523) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk732::<F>(t3520, t439, t447);
    (t3479, t3483, t3491, t3495, t3496, t3503, t3510, t3519, t3520, t3521, t3522, t3523)
}
