//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 564/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk564(t1095: f64, t398: f64, t4521: f64, t384: f64, t1441: f64, t997: f64, t1451: f64, t495: f64, t879: f64, t1089: f64, t175: f64, t1429: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4523 = t398 * t1095 * t4521;
    let t4524 = t384 * t4523;
    let t4532 = 0.16006300097412701803e-1_f64 * t997 * t1441;
    let t4538 = t997 * t1451;
    let t4555 = t495 * t879;
    let t4557 = t1089 * t175 * t4555;
    let t4558 = t384 * t4557;
    let t4561 = 0.40015750243531754508e-1_f64 * t997 * t1429;
    (t4523, t4524, t4532, t4538, t4555, t4557, t4558, t4561)
}
