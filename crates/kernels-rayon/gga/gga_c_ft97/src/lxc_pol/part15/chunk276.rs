//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 276/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk276(t1091: f64, t319: f64, t835: f64, t1212: f64, t840: f64, t1147: f64, t848: f64, t852: f64, t192: f64, t462: f64, t847: f64, t92: f64) -> (f64, f64, f64, f64, f64) {
    let t1221 = t835 * t319 * t1091;
    let t1225 = t840 * t319 * t1212;
    let t1228 = t848 * t1147;
    let t1231 = t852 * t1212;
    let t1232 = t192 * t1231;
    let t1234 = -t847 - t462 * t1228 / 3.0_f64 - t92 * t1232;
    (t1221, t1225, t1228, t1232, t1234)
}
