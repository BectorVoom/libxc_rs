//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk997;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk998;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk999;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1000;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta156<F: Float>(t225: F, t3552: F, t480: F, t371: F, t482: F, t676: F, t481: F, t1231: F, t1256: F, t1247: F, t1261: F, t1266: F, t3591: F, t3600: F, t3606: F, t3610: F, t3613: F, t3620: F, t3625: F, t3631: F, t3637: F, t3640: F, t3644: F, t3647: F, t484: F, t3584: F, t372: F, t3555: F, t3566: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3650, t3651, t3655) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk997::<F>(t225, t3552, t480, t371, t482, t676);
        let (t3657, t3658, t3660) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk998::<F>(t3655, t481, t1231, t1256, t1247, t1261, t1266, t3591, t3600, t3606, t3610, t3613, t3620, t3625, t3631, t3637, t3640, t3644, t3647, t3651, t484);
        let (t3661, t3663, t3666) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk999::<F>(t3584, t482, t371, t372, t225, t3555);
        let t3667 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1000::<F>(t3666, t480);
        let t3670 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1001::<F>(t225, t3566);
    (t3650, t3651, t3655, t3657, t3658, t3660, t3661, t3663, t3666, t3667, t3670)
}
