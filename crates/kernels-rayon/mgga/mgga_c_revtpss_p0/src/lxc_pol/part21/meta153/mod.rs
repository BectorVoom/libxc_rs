//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk977;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk978;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk979;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta153(t3140: f64, t460: f64, t1242: f64, t472: f64, t474: f64, t3147: f64, t479: f64, t1248: f64, t482: f64, t471: f64, t3153: f64, t1042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3594, t3596, t3597, t3598, t3599, t3600, t3601) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk977(t3140, t460, t1242, t472, t474, t3147, t479, t1248);
        let (t3602, t3603) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk978(t3601, t482, t471);
        let t3604 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk979(t3153, t3603);
        let (t3605, t3606) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk980(t3602, t3604, t1042);
    (t3594, t3596, t3597, t3598, t3599, t3600, t3601, t3602, t3603, t3604, t3605, t3606)
}
