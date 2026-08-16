//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 618/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk618(t1113: f64, t2426: f64, t51: f64, t6032: f64, t1092: f64, t1771: f64, t222: f64, t226: f64, t236: f64, t1127: f64, t694: f64, t3724: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13491 = t2426 * t1113;
    let t13519 = t6032 * t51;
    let t13538 = t1771 * t1092;
    let t13580 = t222 * t226;
    let t13581 = t236 * t1113;
    let t13582 = t13580 * t13581;
    let t13585 = t694 * t1127;
    let t13586 = t3724 * t13585;
    (t13491, t13519, t13538, t13580, t13581, t13582, t13586)
}
