//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 94/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk94(t169: f64, t279: f64, t296: f64, t299: f64, t247: f64, t249: f64, t270: f64, t33: f64, t178: f64, t260: f64, t110: f64, t271: f64) -> (f64, f64, f64, f64) {
    let t301 = t279 * t296 * t169 * t299;
    let t304 = -t33 + t247 + t249 + 0.76905262301422242837e-2_f64 * t270 * t301;
    let t305 = t260 * t178;
    let t306 = t110 * t271;
    (t301, t304, t305, t306)
}
