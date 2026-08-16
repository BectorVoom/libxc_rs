//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk654;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk655;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk656;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk657;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta113(t3151: f64, t373: f64, t73: f64, t357: f64, t1042: f64, t1036: f64, t3148: f64, t3141: f64, t1038: f64, t1052: f64, t1033: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3152, t3153) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk654(t3151, t373, t73);
        let t3154 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk655(t357);
        let (t3155, t3156, t3157) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk656(t3153, t3154, t3152, t1042);
        let (t3160, t3161, t3162, t3163, t3164) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk657(t1036, t3148, t3141, t3153, t357, t3152, t1042);
        let (t3168, t3169) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk658(t1038, t1052, t1036, t1033);
    (t3153, t3154, t3155, t3156, t3157, t3160, t3161, t3162, t3163, t3164, t3168, t3169)
}
