//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 997/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk997(t13502: f64, t532: f64, t1581: f64, t3670: f64, t1588: f64, t1008: f64, t4894: f64, t14106: f64, t537: f64, t1576: f64, t14283: f64, t1569: f64, t3228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16674 = t13502 * t532;
    let t16676 = t3670 * t1581;
    let t16678 = t3670 * t1588;
    let t16680 = t1008 * t4894;
    let t16686 = t14106 * t537;
    let t16688 = t3670 * t1576;
    let t16690 = t14283 * t532;
    let t16692 = t3228 * t1569;
    (t16674, t16676, t16678, t16680, t16686, t16688, t16690, t16692)
}
