//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1557;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1558;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta301(t12077: f64, t342: f64, t12051: f64, t3154: f64, t3298: f64, t989: f64, t4980: f64, t994: f64, t4995: f64, t1043: f64, t3153: f64, t3046: f64, t3286: f64, t3057: f64, t1071: f64, t1086: f64, t3316: f64, t11239: f64, t11627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12078, t12079, t12116, t12122) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1556(t12077, t342, t12051, t3154, t3298, t989, t4980, t994);
        let t12127 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1557(t4995, t994);
        let (t12131, t12146, t12149) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1558(t1043, t3153, t3046, t3286, t3057);
        let (t12153, t12154, t12160, t12166) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1559(t1071, t1086, t994, t3316, t989, t11239, t11627);
    (t12078, t12079, t12116, t12122, t12127, t12131, t12146, t12149, t12153, t12154, t12160, t12166)
}
