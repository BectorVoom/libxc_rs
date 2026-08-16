//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 623/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk623(t2568: f64, t737: f64, t762: f64, t2486: f64, t2492: f64, t265: f64, t9802: f64, t1140: f64, t8232: f64, t1170: f64, t1263: f64, t8640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14175 = t737 * t2568;
    let t14182 = t737 * t762;
    let t14187 = t2486 * t762;
    let t14196 = t2492 * t265;
    let t14200 = t9802 * t265;
    let t14224 = t8232 * t1140;
    let t14233 = t8232 * t1170;
    let t14431 = t8640 * t1263;
    (t14175, t14182, t14187, t14196, t14200, t14224, t14233, t14431)
}
