//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 627/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk627(t336: f64, t368: f64, t4838: f64, t3237: f64, t532: f64, t1008: f64, t1581: f64, t1077: f64, t6: f64, t386: f64, t535: f64, t1574: f64, t1579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4840 = t336 * t368 * t4838;
    let t4843 = t3237 * t532;
    let t4846 = 0.85748036236139473944e-3_f64 * t1008 * t1581;
    let t4847 = t6 * t1077;
    let t4849 = t386 * t4847 * t535;
    let t4853 = t386 * t1574 * t1579;
    (t4840, t4843, t4846, t4847, t4849, t4853)
}
