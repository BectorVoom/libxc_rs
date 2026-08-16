//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1052/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1052(t4389: f64, t5143: f64, t1165: f64, t1532: f64, t301: f64, t3194: f64, t4162: f64, t4542: f64, t997: f64, t12719: f64, t527: f64, t3361: f64, t3809: f64, t540: f64) -> (f64, f64, f64, f64, f64) {
    let t18426 = t4389 * t5143;
    let t18436 = t3194 * t1165 * t1532 * t4162 * t301;
    let t18458 = t997 * t4542;
    let t18460 = t12719 * t527;
    let t18475 = t3361 * t1165 * t540 * t3809;
    (t18426, t18436, t18458, t18460, t18475)
}
