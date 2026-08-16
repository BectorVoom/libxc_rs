//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 909/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk909(t1021: f64, t19589: f64, t1092: f64, t1121: f64, t6496: f64, t3218: f64, t1096: f64, t3203: f64, t6276: f64, t3202: f64, t3200: f64, t2822: f64, t6505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19590 = t1021 * t19589;
    let t19591 = t1092 * t19590;
    let t19593 = t6496 * t1121;
    let t19594 = t3218 * t19593;
    let t19595 = t1096 * t19594;
    let t19596 = t1092 * t19595;
    let t19599 = t3203 * t6276 * t1121;
    let t19600 = t3202 * t19599;
    let t19601 = t3200 * t19600;
    let t19603 = t2822 * t6505;
    (t19591, t19593, t19596, t19599, t19601, t19603)
}
