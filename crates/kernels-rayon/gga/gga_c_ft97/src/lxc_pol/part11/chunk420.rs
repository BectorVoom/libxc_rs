//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 420/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk420(t2281: f64, t2282: f64, t637: f64, t1638: f64, t1640: f64, t1645: f64, t1649: f64, t1653: f64, t2008: f64, t2011: f64) -> (f64, f64) {
    let t2284 = t637 * t2281 * t2282;
    let t2289 = 0.19257444444444444444e0_f64 * t1638;
    let t2294 = -0.117377e0_f64 * t2008 + 0.234754e0_f64 * t2011 + t2289 + 0.9628722222222222222e-1_f64 * t1640 - 0.9628722222222222222e-1_f64 * t1645 + 0.28886166666666666666e0_f64 * t1649 - 0.14443083333333333333e0_f64 * t1653;
    (t2284, t2294)
}
