//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 293/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk293(t1212: f64, t852: f64, t192: f64, t1228: f64, t462: f64, t847: f64, t92: f64, t845: f64, t91: f64, t1188: f64, t1215: f64, t860: f64) -> (f64, f64, f64, f64) {
    let t1231 = t852 * t1212;
    let t1232 = t192 * t1231;
    let t1234 = -t847 - t462 * t1228 / 3.0_f64 - t92 * t1232;
    let t1236 = t91 * t845 * t1234;
    let t1240 = t1236 / 6.0_f64 - t860 - t1188 / 9.0_f64 - t1215 / 3.0_f64;
    (t1232, t1234, t1236, t1240)
}
