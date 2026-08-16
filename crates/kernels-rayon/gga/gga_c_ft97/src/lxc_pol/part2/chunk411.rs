//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 411/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk411(t1640: f64, t1645: f64, t1649: f64, t1653: f64, t2008: f64, t2011: f64, t2289: f64, t637: f64, t639: f64, t2251: f64, t2254: f64, t2256: f64, t2261: f64, t2265: f64, t2268: f64, t2273: f64, t2277: f64, t2284: f64, t631: f64) -> (f64, f64, f64) {
    let t2294 = -0.117377e0_f64 * t2008 + 0.234754e0_f64 * t2011 + t2289 + 0.9628722222222222222e-1_f64 * t1640 - 0.9628722222222222222e-1_f64 * t1645 + 0.28886166666666666666e0_f64 * t1649 - 0.14443083333333333333e0_f64 * t1653;
    let t2296 = t637 * t639 * t2294;
    let t2299 = -t2251 - 2.0_f64 / 9.0_f64 * t2254 - 2.0_f64 / 3.0_f64 * t2256 + t631 * t2261 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t2265 * t2268 - t631 * t2273 / 3.0_f64 + t631 * t2277 / 6.0_f64 - 3.0_f64 / 2.0_f64 * t631 * t2284 + t631 * t2296 / 2.0_f64;
    (t2294, t2296, t2299)
}
