//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta691 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2692;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2693;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta691(t221: f64, t6836: f64, t9921: f64, t3978: f64, t125: f64, t6816: f64, t1399: f64, t3936: f64, t6843: f64, t3938: f64, t5673: f64, t21990: f64, t5674: f64, t13944: f64, t6869: f64, t543: f64, t5591: f64, t13848: f64, t9818: f64, t9816: f64, t13798: f64, t13801: f64, t13810: f64, t13813: f64, t3934: f64, t5671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22068, t22069, t22074, t22076, t22079) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2692(t221, t6836, t9921, t3978, t125, t6816, t1399, t3936, t6843);
        let (t22081, t22085, t22089, t22093, t22096) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2693(t22079, t3936, t3938, t1399, t5673, t21990, t5674, t13944, t6869, t543, t5591);
        let (t22098, t22102, t22105) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2694(t22096, t3936, t5674, t13848, t6869, t9818, t9816, t13798, t13801, t13810, t13813, t22069, t22076, t22081, t22085, t22089, t22093, t3934, t5671);
    (t22068, t22074, t22076, t22079, t22081, t22085, t22089, t22093, t22096, t22098, t22102, t22105)
}
