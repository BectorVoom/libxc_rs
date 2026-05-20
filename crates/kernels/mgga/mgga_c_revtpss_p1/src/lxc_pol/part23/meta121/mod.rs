//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk784;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk785;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk786;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk787;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk788;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk789;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk790;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta121<F: Float>(t1054: F, t1058: F, t1014: F, t2857: F, t1010: F, t614: F, t1016: F, t140: F, t1011: F, t271: F, t905: F, t2852: F, t1071: F, t342: F, t1077: F, t384: F, t225: F, t1086: F, t989: F, t378: F, t994: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3234, t3236, t3241) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk784::<F>(t1054, t1058, t1014, t2857, t1010, t614);
        let (t3245, t3252) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk785::<F>(t1016, t140, t1011, t271, t905);
        let (t3253, t3264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk786::<F>(t2852, t3252, t1071, t342);
        let t3268 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk787::<F>(t1077, t384);
        let t3269 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk788::<F>(t225, t3268);
        let t3278 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk789::<F>(t1086, t989);
        let t3286 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk790::<F>(t1086, t378);
        let t3287 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk791::<F>(t3286, t994);
    (t3234, t3236, t3241, t3245, t3252, t3253, t3264, t3268, t3269, t3278, t3286, t3287)
}
