//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1825/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1825(t3006: f64, t3014: f64, t2873: f64, t910: f64, t11132: f64, t2942: f64, t941: f64, t2986: f64, t960: f64, t2979: f64, t300: f64, t1034: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11524 = t3006 * t3014;
    let t11528 = t910 * t2873;
    let t11534 = 0.55403703703703703703e-1_f64 * t11132;
    let t11548 = t941 * t2942;
    let t11554 = t960 * t2986;
    let t11560 = 0.28842592592592592592e-1_f64 * t11132;
    let t11574 = 0.53272592592592592592e-1_f64 * t11132;
    let t11591 = t300 * t2979;
    let t11626 = t1034 * t1034;
    (t11524, t11528, t11534, t11548, t11554, t11560, t11574, t11591, t11626)
}
