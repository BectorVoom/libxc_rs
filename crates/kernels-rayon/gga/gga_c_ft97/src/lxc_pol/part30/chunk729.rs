//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 729/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk729(t33332: f64, t743: f64, t193: f64, t6109: f64, t33286: f64, t33291: f64, t33297: f64, t33305: f64, t33310: f64, t33314: f64, t33318: f64, t33322: f64, t33326: f64, t33330: f64) -> (f64, f64, f64) {
    let t33333 = t743 * t33332;
    let t33335 = t6109 * t193 * t33333;
    let t33337 = 3.0_f64 / 2.0_f64 * t33286 + t33291 + 2.0_f64 / 3.0_f64 * t33297 + 4.0_f64 * t33305 - 2.0_f64 * t33310 - t33314 / 2.0_f64 - t33318 - t33322 / 3.0_f64 - 3.0_f64 * t33326 + 2.0_f64 * t33330 + t33335 / 4.0_f64;
    (t33333, t33335, t33337)
}
