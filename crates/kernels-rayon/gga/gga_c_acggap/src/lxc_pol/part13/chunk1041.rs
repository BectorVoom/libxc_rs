//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1041/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1041(t1459: f64, t1980: f64, t33883: f64, t7458: f64, t1967: f64, t8541: f64, t31038: f64, t527: f64, t8497: f64, t2001: f64, t4528: f64, t1998: f64, t4523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34305 = t1980 * t7458 * t1459 * t33883;
    let t34307 = t1967 * t8541;
    let t34308 = 0.64311027177104605458e-2_f64 * t34307;
    let t34309 = t31038 * t527;
    let t34311 = t1967 * t8497;
    let t34312 = 0.25724410870841842184e-2_f64 * t34311;
    let t34313 = t2001 * t4528;
    let t34315 = t1998 * t4523;
    (t34305, t34308, t34309, t34312, t34313, t34315)
}
