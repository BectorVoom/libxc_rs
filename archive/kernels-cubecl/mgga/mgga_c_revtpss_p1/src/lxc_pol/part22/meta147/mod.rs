//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta147 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk972;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk973;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk974;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk975;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk976;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta147<F: Float>(t3453: F, t3479: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t448: F, t1175: F, t1179: F, t1178: F, t444: F, t439: F, t1187: F, t1188: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3480, t3483, t3488) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk972::<F>(t3453, t3479, t3356, t3358, t3365, t3370, t3374);
        let (t3489, t3491) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk973::<F>(t3488, t448, t1175, t1179);
        let (t3494, t3495) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk974::<F>(t1178, t444);
        let t3496 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk975::<F>(t3495, t439);
        let t3497 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk976::<F>(t1187);
        let t3498 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk977::<F>(t1188, t3497);
    (t3480, t3483, t3488, t3489, t3491, t3494, t3495, t3496, t3497, t3498)
}
