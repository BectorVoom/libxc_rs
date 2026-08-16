//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1557;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1558;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta301<F: Float>(t12077: F, t342: F, t12051: F, t3154: F, t3298: F, t989: F, t4980: F, t994: F, t4995: F, t1043: F, t3153: F, t3046: F, t3286: F, t3057: F, t1071: F, t1086: F, t3316: F, t11239: F, t11627: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12078, t12079, t12116, t12122) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1556::<F>(t12077, t342, t12051, t3154, t3298, t989, t4980, t994);
        let t12127 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1557::<F>(t4995, t994);
        let (t12131, t12146, t12149) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1558::<F>(t1043, t3153, t3046, t3286, t3057);
        let (t12153, t12154, t12160, t12166) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1559::<F>(t1071, t1086, t994, t3316, t989, t11239, t11627);
    (t12078, t12079, t12116, t12122, t12127, t12131, t12146, t12149, t12153, t12154, t12160, t12166)
}
