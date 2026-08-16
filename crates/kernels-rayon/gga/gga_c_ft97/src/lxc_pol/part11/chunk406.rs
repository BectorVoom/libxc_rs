//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 406/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk406(t2190: f64, t574: f64, t605: f64, t1882: f64, t571: f64, t379: f64, t569: f64, t616: f64, t1651: f64, t167: f64, t143: f64, t1642: f64) -> (f64, f64, f64, f64, f64) {
    let t2192 = t574 * t605 * t2190;
    let t2195 = t1882 * t571;
    let t2198 = t569 * t616 * t379;
    let t2202 = t569 * t167 * t1651;
    let t2205 = t1642 * t143;
    (t2192, t2195, t2198, t2202, t2205)
}
