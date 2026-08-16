//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1052;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1053;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1054;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1055;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1056;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1057;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta243(t1096: f64, t3270: f64, t11121: f64, t1071: f64, t3046: f64, t268: f64, t271: f64, t7021: f64, t2435: f64, t907: f64, t2854: f64, t689: f64, t2859: f64, t2863: f64, t159: f64, t3181: f64, t2851: f64, t631: f64, t10356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11122, t11123, t11128, t11132) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1052(t1096, t3270, t11121, t1071, t3046, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1053(t11132, t2435, t907);
        let t11136 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1054(t2854, t689);
        let t11138 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1055(t2859, t689);
        let t11140 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1056(t2863, t689);
        let (t11142, t11144) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1057(t159, t3181, t2851, t631);
        let t11145 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1058(t10356, t11144);
    (t11122, t11123, t11128, t11132, t11133, t11134, t11136, t11138, t11140, t11142, t11144, t11145)
}
