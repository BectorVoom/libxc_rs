//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1169/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1169(t1992: f64, t30692: f64, t5720: f64, t7842: f64, t30364: f64, t5147: f64, t8901: f64, t1181: f64, t2068: f64, t23045: f64, t604: f64, t30689: f64, t4967: f64) -> (f64, f64, f64, f64, f64) {
    let t36004 = t30692 * t7842 * t1992 * t5720;
    let t36005 = 0.10482697429868050689e-2_f64 * t36004;
    let t36006 = t30364 * t5147;
    let t36007 = 0.17149607247227894789e-2_f64 * t36006;
    let t36010 = t30692 * t7842 * t1992 * t8901;
    let t36011 = 0.10482697429868050689e-2_f64 * t36010;
    let t36014 = t2068 * t1181 * t604 * t23045;
    let t36017 = t30689 * t4967;
    (t36005, t36007, t36011, t36014, t36017)
}
