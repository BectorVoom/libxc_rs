//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 799/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk799(t2822: f64, t2831: f64, t2820: f64, t2840: f64, t86: f64, t2847: f64, t3225: f64, t283: f64, t3201: f64, t982: f64, t1018: f64, t1085: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9383 = t2822 * t2831;
    let t9386 = t86 * t2820 * t2840;
    let t9387 = t9386 * t2847;
    let t9409 = t3225 * sigma0;
    let t9410 = t9409 * t283;
    let t9415 = t3201 * t982;
    let t9423 = t1018 * t1085;
    (t9383, t9386, t9387, t9409, t9410, t9415, t9423)
}
