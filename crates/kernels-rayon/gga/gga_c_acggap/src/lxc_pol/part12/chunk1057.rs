//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1057/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1057(t1988: f64, t8549: f64, t1095: f64, t1426: f64, t34045: f64, t598: f64, t1980: f64, t4806: f64, t7476: f64, t7799: f64, t8555: f64, t13287: f64, t2302: f64, t31195: f64, t3196: f64) -> (f64, f64, f64, f64, f64) {
    let t34794 = t1988 * t8549;
    let t34798 = t598 * t1426 * t1095 * t34045;
    let t34802 = t1980 * t7476 * t1095 * t4806;
    let t34804 = t7799 * t8555;
    let t34817 = t31195 * t13287 * t2302 * t3196;
    (t34794, t34798, t34802, t34804, t34817)
}
