//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 871/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk871(t13571: f64, t200: f64, t2379: f64, t2382: f64, t2417: f64, t222: f64, t226: f64, t1113: f64, t236: f64, t1127: f64, t694: f64, t3724: f64) -> (f64, f64, f64, f64) {
    let t13572 = t13571 * t200;
    let t13577 = t2379 * t2417 * t2382;
    let t13580 = t222 * t226;
    let t13581 = t236 * t1113;
    let t13582 = t13580 * t13581;
    let t13585 = t694 * t1127;
    let t13586 = t3724 * t13585;
    (t13572, t13577, t13582, t13586)
}
