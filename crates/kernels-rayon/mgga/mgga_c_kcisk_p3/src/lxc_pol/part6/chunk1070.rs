//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1070/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1070(t30551: f64, t3725: f64, t1212: f64, t30705: f64, t12888: f64, t14831: f64, t19100: f64, t25590: f64, t25601: f64, t25609: f64, t30569: f64, t30572: f64, t30592: f64, t30595: f64, t30599: f64, t30603: f64) -> (f64, f64, f64, f64) {
    let t31581 = t30551 * t3725;
    let t31584 = t30705 * t1212;
    let t31587 = t30551 * t12888;
    let t31603 = -t14831 - 0.2283111111111111111e-1_f64 * t19100 + 0.11415555555555555555e-1_f64 * t25590 - 0.34246666666666666665e-1_f64 * t25601 + 0.17123333333333333333e-1_f64 * t25609 - 0.19025925925925925925e-1_f64 * t30592 + 0.68493333333333333331e-1_f64 * t30595 - 0.34246666666666666665e-1_f64 * t30569 - 0.10274e0_f64 * t30599 + 0.10274e0_f64 * t30572 - 0.17123333333333333333e-1_f64 * t30603;
    (t31581, t31584, t31587, t31603)
}
