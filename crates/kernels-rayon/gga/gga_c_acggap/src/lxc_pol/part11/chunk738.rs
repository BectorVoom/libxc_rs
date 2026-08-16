//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 738/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk738(t7760: f64, t1426: f64, t2085: f64, t429: f64, t598: f64, t368: f64, t7470: f64, t7476: f64, t7483: f64, t1980: f64, t1967: f64, t1973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7761 = 0.10718504529517434243e-2_f64 * t7760;
    let t7763 = t1426 * t429 * t2085;
    let t7764 = t598 * t7763;
    let t7767 = t1426 * t368 * t7470;
    let t7768 = t598 * t7767;
    let t7770 = t7476 * t7483;
    let t7771 = t1980 * t7770;
    let t7772 = 0.7145669686344956162e-3_f64 * t7771;
    let t7773 = t1967 * t1973;
    (t7761, t7763, t7764, t7767, t7768, t7770, t7771, t7772, t7773)
}
