//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 724/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk724(t140: f64, t20651: f64, t550: f64, t133: f64, t2001: f64, t20576: f64, t20578: f64, t20580: f64, t20583: f64, t20586: f64, t20632: f64, t20636: f64, t3392: f64, t3393: f64, t4710: f64) -> f64 {
    let t141 = 0.1e-59_f64 < t140;
    let t20652 = t550 * t20651;
    let t20653 = t133 * t20652;
    let t20655 = piecewise3(t141, 6.0_f64 * t3392 * t3393 * t4710 + 12.0_f64 * t2001 * t20580 - 6.0_f64 * t2001 * t20583 - 6.0_f64 * t2001 * t20586 + 6.0_f64 * t20576 - 6.0_f64 * t20578 + 2.0_f64 * t20632 - 6.0_f64 * t20636 - t20653, 0.0_f64);
    t20655
}
