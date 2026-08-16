//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1371/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1371(t1146: f64, t8980: f64, t3160: f64, t3169: f64, t1141: f64, t8996: f64, t469: f64, t8995: f64, t454: f64, t3171: f64, t3264: f64, t1179: f64, t27126: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27255 = t8980 * t1146;
    let t27259 = t3160 * t3169;
    let t27266 = t1141 * t8996;
    let t27276 = 1.0_f64 / t8995 / t469;
    let t27277 = t454 * t27276;
    let t27278 = t3171 * t3171;
    let t27286 = t3264 * t3264;
    let t27297 = t1179 * t27126;
    (t27255, t27259, t27266, t27277, t27278, t27286, t27297)
}
