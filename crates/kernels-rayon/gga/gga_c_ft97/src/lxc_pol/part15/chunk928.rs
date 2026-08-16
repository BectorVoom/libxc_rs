//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 928/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk928(t19038: f64, t70474: f64, t287: f64, t5014: f64, t2724: f64, t5260: f64, t1200: f64, t7606: f64, t19106: f64, t800: f64, t4092: f64, t70462: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70475 = t19038 * t70474;
    let t70476 = t5014 * t287;
    let t70487 = t2724 * t5260;
    let t70497 = t1200 * t7606;
    let t70550 = t800 * t19106;
    let t70653 = t4092 * t70462;
    (t70475, t70476, t70487, t70497, t70550, t70653)
}
