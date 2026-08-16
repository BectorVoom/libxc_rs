//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1020/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1020(t34052: f64, t2304: f64, t7780: f64, t7799: f64, t8545: f64, t30260: f64, t8491: f64, t336: f64, t4838: f64, t578: f64, t599: f64, t30402: f64, t31309: f64, t525: f64, t7325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34053 = 0.7145669686344956162e-3_f64 * t34052;
    let t34054 = t7780 * t2304;
    let t34056 = t7799 * t8545;
    let t34058 = 0.13976929906490734252e-1_f64 * t30260;
    let t34059 = t7799 * t8491;
    let t34063 = t578 * t336 * t599 * t4838;
    let t34068 = t31309 * t30402 * t7325 * t525;
    (t34053, t34054, t34056, t34058, t34059, t34063, t34068)
}
