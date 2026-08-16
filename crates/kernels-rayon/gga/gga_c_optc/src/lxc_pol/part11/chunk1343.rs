//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1343/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1343(t43347: f64, t52269: f64, t57327: f64, t57330: f64, t57332: f64, t57335: f64, t57337: f64, t57343: f64, t57346: f64, t57349: f64, t57351: f64, t57527: f64) -> f64 {
    let t58240 = 2.0_f64 / 9.0_f64 * t52269 - t57327 + 4.0_f64 / 9.0_f64 * t43347 - t57330 - t57332 - t57335 + t57337 - t57343 + t57346 + t57349 - t57351 - t57527;
    t58240
}
