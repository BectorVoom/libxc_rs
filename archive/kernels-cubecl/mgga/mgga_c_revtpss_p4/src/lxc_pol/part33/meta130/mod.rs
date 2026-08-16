//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk717;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk718;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk719;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta130<F: Float>(t1086: F, t989: F, t378: F, t994: F, t1071: F, t359: F, t3140: F, t3143: F, t342: F, t335: F, t368: F, t3153: F, t3154: F, t1035: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3278, t3286, t3287, t3291, t3298) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk717::<F>(t1086, t989, t378, t994, t1071, t359, t3140, t3143);
        let (t3299, t3302) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk718::<F>(t3298, t342, t335, t368);
        let t3303 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk719::<F>(t3153, t3302);
        let (t3304, t3316) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk720::<F>(t3154, t3303, t1035, t3140);
    (t3278, t3286, t3287, t3291, t3298, t3299, t3302, t3303, t3304, t3316)
}
