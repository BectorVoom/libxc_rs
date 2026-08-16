//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 877/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk877(t1181: f64, t14575: f64, t599: f64, t7346: f64, t1983: f64, t30262: f64, t4210: f64, t7586: f64, t1170: f64, t8462: f64, t2028: f64, t7599: f64) -> (f64, f64, f64, f64) {
    let t30273 = t7346 * t1181 * t599 * t14575;
    let t30280 = t30262 * t7586 * t1983 * t4210;
    let t30282 = t1170 * t8462;
    let t30307 = t7599 * t2028;
    (t30273, t30280, t30282, t30307)
}
