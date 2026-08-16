//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 652/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk652(t1816: f64, t5137: f64, t639: f64, t1702: f64, t617: f64, t1809: f64, t1620: f64, t661: f64, t1815: f64, t5038: f64, t2677: f64, t5029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5138 = t5137 * t1816;
    let t5139 = t639 * t5138;
    let t5140 = 16.0_f64 / 45.0_f64 * t5139;
    let t5141 = t1702 * t617;
    let t5142 = t1809 * t5141;
    let t5144 = 8.0_f64 / 15.0_f64 * t1620 * t5142;
    let t5145 = t1702 * t661;
    let t5146 = t1815 * t5145;
    let t5148 = 4.0_f64 / 15.0_f64 * t639 * t5146;
    let t5149 = t1809 * t5038;
    let t5151 = 8.0_f64 / 15.0_f64 * t639 * t5149;
    let t5152 = t2677 * t5029;
    (t5138, t5140, t5141, t5142, t5144, t5145, t5146, t5148, t5149, t5151, t5152)
}
