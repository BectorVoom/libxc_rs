//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta35 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk262;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk263;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk264;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk265;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk266;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk267;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk268;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk269;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta35(t138: f64, t697: f64, t687: f64, t689: f64, t693: f64, t146: f64, t682: f64, t36: f64, t37: f64, t157: f64, t190: f64, t606: f64, t45: f64, t57: f64, t78: f64, t81: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t698 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk262(t138, t697);
        let t700 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk263(t687, t689, t693, t698);
        let t701 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk264(t146);
        let t702 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk265(t700, t701);
        let t704 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk266(t682, t702);
        let t705 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk267(t36, t37);
        let t706 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk268(t157, t705);
        let t707 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk269(t190, t606);
        let (t709, t716) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk270(t45, t57, t706, t707, t606, t78, t81, zeta_threshold);
    (t698, t700, t701, t702, t704, t705, t706, t707, t709, t716)
}
