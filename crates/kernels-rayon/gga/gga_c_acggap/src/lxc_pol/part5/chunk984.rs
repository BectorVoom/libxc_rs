//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 984/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk984(t4794: f64, t4796: f64, t576: f64, t168: f64, t352: f64, t355: f64, t4353: f64, t721: f64, t4795: f64, t4818: f64, t4822: f64, t13768: f64, t2: f64, t3153: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16233 = t576 * t4794 * t4796;
    let t16236 = t352 * t168 * t355;
    let t16238 = t16236 * t4353 * t721;
    let t16241 = t4795 * t4818 * t721;
    let t16244 = t4795 * t4822 * t721;
    let t16249 = t3153 * t13768 * t495 * t2;
    (t16233, t16236, t16238, t16241, t16244, t16249)
}
