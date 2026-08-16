//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 619/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk619(t2229: f64, t19: f64, t2218: f64, t2220: f64, t2222: f64, t2224: f64, t2226: f64, t2228: f64, t601: f64, t604: f64) -> (f64, f64, f64, f64) {
    let t2230 = 1.0_f64 / t2229;
    let t2232 = 0.9492e2_f64 * t19 * t2230;
    let t2233 = t2218 - t2220 + t2222 - t2224 + t2226 - t2228 + t2232;
    let t2235 = t601 * t604;
    (t2230, t2232, t2233, t2235)
}
