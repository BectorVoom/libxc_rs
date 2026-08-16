//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 543/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk543(t1426: f64, t175: f64, t3491: f64, t384: f64, t1137: f64, t962: f64, t1131: f64, t322: f64, t1095: f64, t398: f64, t177: f64, t414: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3493 = t1426 * t175 * t3491;
    let t3494 = t384 * t3493;
    let t3504 = t1137 * t962;
    let t3529 = t1131 * t322;
    let t3531 = t398 * t1095 * t3529;
    let t3532 = t384 * t3531;
    let t3551 = 0.30011812682648815881e-2_f64 * t980 * t414 * t177;
    (t3493, t3494, t3504, t3529, t3531, t3532, t3551)
}
