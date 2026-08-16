//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 607/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk607(t1906: f64, t8392: f64, t1922: f64, t432: f64, t452: f64, t1755: f64, t499: f64, t110: f64, t8183: f64, t447: f64, t7966: f64, t1873: f64, t1882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8393 = t8392 * t1906;
    let t8396 = t452 * t1922 * t432;
    let t8399 = t452 * t499 * t1755;
    let t8402 = t452 * t110 * t8183;
    let t8406 = t447 * t110 * t7966;
    let t8409 = t1882 * t1873;
    (t8393, t8396, t8399, t8402, t8406, t8409)
}
