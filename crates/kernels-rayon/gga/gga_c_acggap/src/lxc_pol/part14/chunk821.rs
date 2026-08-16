//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 821/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk821(t1089: f64, t5011: f64, t9552: f64, t598: f64, t1817: f64, t7733: f64, t2288: f64, t4643: f64, t137: f64, t1795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9554 = t1089 * t5011 * t9552;
    let t9555 = t598 * t9554;
    let t9557 = t7733 * t1817;
    let t9560 = t1089 * t4643 * t2288;
    let t9561 = t598 * t9560;
    let t9563 = t137 * t1795;
    (t9554, t9555, t9557, t9560, t9561, t9563)
}
