//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1858;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1859;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1860;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1861;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta356(t12077: f64, t342: f64, t1086: f64, t3043: f64, t3298: f64, t989: f64, t4980: f64, t994: f64, t4995: f64, t1043: f64, t3153: f64, t3133: f64, t4982: f64, t3046: f64, t3286: f64, t3057: f64, t1071: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12078, t12097, t12116, t12122) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1858(t12077, t342, t1086, t3043, t3298, t989, t4980, t994);
        let t12127 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1859(t4995, t994);
        let (t12131, t12132, t12146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1860(t1043, t3153, t3133, t4982, t3046, t3286);
        let t12149 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1861(t3057, t3286);
        let (t12153, t12154) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1862(t1071, t1086, t994);
    (t12078, t12097, t12116, t12122, t12127, t12131, t12132, t12146, t12149, t12153, t12154)
}
