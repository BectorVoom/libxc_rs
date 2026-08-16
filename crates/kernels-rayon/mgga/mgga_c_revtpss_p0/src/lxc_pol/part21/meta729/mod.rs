//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta729 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2572;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta729(t221: f64, t4019: f64, t47293: f64, t9995: f64, t9905: f64, t9976: f64, t9984: f64, t3978: f64, t9921: f64, t3926: f64, t9909: f64, t3930: f64, t9901: f64, t2661: f64, t5675: f64, t9929: f64, t9934: f64, t9775: f64, t9981: f64, t1398: f64, t3992: f64, t4010: f64, t9956: f64, t3938: f64, t47218: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47296, t47298, t47302, t47304, t47306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2572(t221, t4019, t47293, t9995, t9905, t9976, t9984, t3978, t9921, t3926, t9909, t3930, t9901);
        let (t47318, t47320, t47325, t47329) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2573(t2661, t5675, t9929, t9934, t9775, t9981, t1398, t3992, t4010, t9956, t3938, t47218);
    (t47296, t47298, t47302, t47304, t47306, t47318, t47320, t47325, t47329)
}
