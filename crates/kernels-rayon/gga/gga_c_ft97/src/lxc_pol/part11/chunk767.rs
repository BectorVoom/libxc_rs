//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 767/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk767(t10388: f64, t799: f64, t27: f64, t89: f64, t2740: f64, t375: f64, t10: f64, t296: f64, t3050: f64, t1636: f64, t825: f64, t2660: f64, t9571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10389 = t799 * t10388;
    let t10391 = t89 * t27 * t10389;
    let t10394 = t89 * t375 * t2740;
    let t10397 = t10 * t3050 * t296;
    let t10398 = 14.0_f64 / 81.0_f64 * t10397;
    let t10400 = t89 * t1636 * t825;
    let t10402 = t2660 * t9571;
    (t10389, t10391, t10394, t10397, t10398, t10400, t10402)
}
