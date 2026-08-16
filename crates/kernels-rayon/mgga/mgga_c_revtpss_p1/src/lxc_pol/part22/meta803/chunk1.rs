//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2905/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2905(t1331: f64, t9855: f64, t2619: f64, t9563: f64, t3825: f64, t9586: f64, t14: f64, t27: f64, t521: f64, t525: f64, t9603: f64, t527: f64, t9615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47007 = t9855 * t1331;
    let t47009 = t9563 * t2619;
    let t47011 = t3825 * t9586;
    let t47016 = t14 * t27 * t521;
    let t47025 = 1.0_f64 / t525 / t9603;
    let t47040 = 1.0_f64 / t527 / t9615;
    (t47007, t47009, t47011, t47016, t47025, t47040)
}
