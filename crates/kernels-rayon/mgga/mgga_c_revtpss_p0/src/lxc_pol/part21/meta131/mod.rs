//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta131 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk845;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk846;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk847;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk848;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk849;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk850;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta131(t3090: f64, t3114: f64, t373: f64, t66: f64, t828: f64, t1043: f64, t999: f64, t1045: f64, t1032: f64, t989: f64, t1040: f64, t1024: f64, t1062: f64, t1065: f64, t906: f64, t1042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3115 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk845(t3090, t3114);
        let t3116 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk846(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk847(t3116, t828);
        let (t3118, t3119, t3120) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk848(t1043, t999, t1045, t3117);
        let (t3123, t3124) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk849(t1032, t989, t1040);
        let t3127 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk850(t1024, t1062);
        let (t3128, t3129, t3130) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk851(t1065, t999, t906, t1042);
    (t3115, t3116, t3117, t3118, t3119, t3120, t3123, t3124, t3127, t3128, t3129, t3130)
}
