//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 968/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk968(t1967: f64, t8541: f64, t31038: f64, t527: f64, t8497: f64, t1998: f64, t4523: f64, t7676: f64, t8689: f64, t1181: f64, t5087: f64, t604: f64, t7426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34307 = t1967 * t8541;
    let t34308 = 0.64311027177104605458e-2_f64 * t34307;
    let t34309 = t31038 * t527;
    let t34311 = t1967 * t8497;
    let t34312 = 0.25724410870841842184e-2_f64 * t34311;
    let t34315 = t1998 * t4523;
    let t34317 = t7676 * t8689;
    let t34336 = t7426 * t1181 * t604 * t5087;
    (t34308, t34309, t34312, t34315, t34317, t34336)
}
