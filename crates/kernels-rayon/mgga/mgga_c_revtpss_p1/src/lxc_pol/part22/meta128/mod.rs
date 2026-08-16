//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta128 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk857;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk858;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk859;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk860;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk861;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk862;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta128(t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t996: f64, t221: f64, t346: f64, t696: f64, t345: f64, t2270: f64, t344: f64, t1003: f64, t1007: f64, t360: f64, t365: f64, t1038: f64, t72: f64, t1087: f64, t1066: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3070, t3075) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk857(t2846, t2848, t2855, t2860, t2864);
        let t3076 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk858(t3075, t996);
        let (t3080, t3082, t3083, t3086, t3088) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk859(t221, t346, t696, t345, t2270, t344, t1003, t1007, t360, t365);
        let t3089 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk860(t1038, t72);
        let t3090 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk861(t3088, t3089);
        let t3091 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk862(t1087, t3090);
        let t3092 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk863(t1066, t828);
    (t3070, t3075, t3076, t3080, t3082, t3083, t3086, t3088, t3089, t3090, t3091, t3092)
}
