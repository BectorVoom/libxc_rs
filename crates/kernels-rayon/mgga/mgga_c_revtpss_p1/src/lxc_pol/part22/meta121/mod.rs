//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk815;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk816;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk817;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta121(t2853: f64, t2908: f64, t141: f64, t2858: f64, t930: f64, t2862: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t2882: f64, t2890: f64, t2892: f64, t2898: f64, t2900: f64, t2905: f64, t2906: f64, t935: f64, t915: f64, t913: f64, t275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2909, t2910, t2912, t2913, t2915, t2916, t2918) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk815(t2853, t2908, t141, t2858, t930, t2862, t2848, t2855, t2860, t2864, t2882, t2890, t2892, t2898, t2900, t2905, t2906);
        let (t2919, t2921, t2922) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk816(t2918, t935, t915, t913);
        let t2923 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk817(t2922);
        let t2924 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk818(t275, t2923);
    (t2909, t2910, t2912, t2913, t2915, t2916, t2918, t2919, t2921, t2922, t2923, t2924)
}
