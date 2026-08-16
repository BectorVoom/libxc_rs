//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1292/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1292(t4793: f64, t2399: f64, t4786: f64, t23844: f64, t2382: f64, t23913: f64, t39411: f64, t49385: f64, t49387: f64, t56966: f64, t56978: f64, t56981: f64, t56984: f64, t57024: f64, t57057: f64, t57060: f64, t57063: f64) -> (f64, f64, f64, f64, f64) {
    let t57065 = t4793 * t4793;
    let t57066 = t2399 * t57065;
    let t57068 = t4786 * t4786;
    let t57069 = t23844 * t57068;
    let t57071 = t2382 * t57065;
    let t57073 = t23913 * t57068;
    let t57086 = -t57057 / 3.0_f64 + 8.0_f64 * t57060 - 12.0_f64 * t56978 + 2.0_f64 * t57063 - 16.0_f64 / 9.0_f64 * t49385 + 8.0_f64 / 3.0_f64 * t49387 + 8.0_f64 / 3.0_f64 * t56981 - 8.0_f64 / 9.0_f64 * t56984 - 8.0_f64 / 9.0_f64 * t39411 - 8.0_f64 * t57024 + 8.0_f64 * t56966;
    (t57066, t57069, t57071, t57073, t57086)
}
