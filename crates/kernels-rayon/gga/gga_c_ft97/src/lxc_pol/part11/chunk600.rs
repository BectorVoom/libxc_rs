//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 600/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk600(t8299: f64, t8335: f64, t457: f64, t91: f64, t369: f64, t631: f64, t637: f64, t7242: f64, t96: f64, t1767: f64, t473: f64, t1766: f64, t1808: f64) -> (f64, f64, f64, f64, f64) {
    let t8336 = t8299 + t8335;
    let t8338 = t91 * t457 * t8336;
    let t8345 = 1.0_f64 / t96 / t631 / t637 / t369 / t7242 / 4.0_f64;
    let t8346 = t1767 * t473;
    let t8348 = t91 * t8345 * t8346;
    let t8352 = t91 * t1766 * t473 * t1808;
    (t8336, t8338, t8345, t8348, t8352)
}
