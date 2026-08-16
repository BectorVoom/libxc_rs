//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2089;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta541(t22096: f64, t3936: f64, t5674: f64, t13848: f64, t6869: f64, t9818: f64, t9816: f64, t13798: f64, t13801: f64, t13810: f64, t13813: f64, t22069: f64, t22076: f64, t22081: f64, t22085: f64, t22089: f64, t22093: f64, t3934: f64, t5671: f64, t22046: f64, t3938: f64, t5659: f64, t5673: f64, t1399: f64, t125: f64, t6836: f64, t9955: f64, t1413: f64, t6816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22098, t22102, t22103, t22105) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2089(t22096, t3936, t5674, t13848, t6869, t9818, t9816, t13798, t13801, t13810, t13813, t22069, t22076, t22081, t22085, t22089, t22093, t3934, t5671);
        let (t22107, t22111, t22115, t22120, t22125) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2090(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
    (t22098, t22102, t22103, t22105, t22107, t22111, t22115, t22120, t22125)
}
