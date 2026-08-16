//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1097/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1097(t1967: f64, t8838: f64, t4352: f64, t535: f64, t598: f64, t7656: f64, t1089: f64, t12473: f64, t2288: f64, t4360: f64, t7741: f64, t13287: f64, t34823: f64, t34828: f64) -> (f64, f64, f64, f64, f64) {
    let t35515 = t1967 * t8838;
    let t35519 = t598 * t4352 * t535 * t7656;
    let t35523 = t598 * t1089 * t12473 * t2288;
    let t35529 = t7741 * t4360;
    let t35535 = t34823 * t13287 * t34828;
    (t35515, t35519, t35523, t35529, t35535)
}
