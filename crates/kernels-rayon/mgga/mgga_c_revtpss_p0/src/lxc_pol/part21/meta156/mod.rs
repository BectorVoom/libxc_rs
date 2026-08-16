//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk997;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk998;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk999;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1000;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta156(t225: f64, t3552: f64, t480: f64, t371: f64, t482: f64, t676: f64, t481: f64, t1231: f64, t1256: f64, t1247: f64, t1261: f64, t1266: f64, t3591: f64, t3600: f64, t3606: f64, t3610: f64, t3613: f64, t3620: f64, t3625: f64, t3631: f64, t3637: f64, t3640: f64, t3644: f64, t3647: f64, t484: f64, t3584: f64, t372: f64, t3555: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3650, t3651, t3655) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk997(t225, t3552, t480, t371, t482, t676);
        let (t3657, t3658, t3660) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk998(t3655, t481, t1231, t1256, t1247, t1261, t1266, t3591, t3600, t3606, t3610, t3613, t3620, t3625, t3631, t3637, t3640, t3644, t3647, t3651, t484);
        let (t3661, t3663, t3666) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk999(t3584, t482, t371, t372, t225, t3555);
        let t3667 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1000(t3666, t480);
        let t3670 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1001(t225, t3566);
    (t3650, t3651, t3655, t3657, t3658, t3660, t3661, t3663, t3666, t3667, t3670)
}
