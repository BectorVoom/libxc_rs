//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1016/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1016(t7637: f64, t8491: f64, t1967: f64, t8536: f64, t4708: f64, t7561: f64, t4439: f64, t7822: f64, t4681: f64, t4443: f64, t30543: f64, t8661: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34011 = t7637 * t8491;
    let t34013 = t1967 * t8536;
    let t34014 = 0.64311027177104605458e-2_f64 * t34013;
    let t34015 = t7561 * t4708;
    let t34017 = t7822 * t4439;
    let t34019 = t7822 * t4681;
    let t34021 = t7822 * t4443;
    let t34023 = t30543 * t8661;
    (t34011, t34014, t34015, t34017, t34019, t34021, t34023)
}
