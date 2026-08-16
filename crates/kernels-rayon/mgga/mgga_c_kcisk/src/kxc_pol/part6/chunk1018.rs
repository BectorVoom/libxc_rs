//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1018/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1018(t30691: f64, t30704: f64, t1203: f64, t1212: f64, t5770: f64, t8378: f64, t13110: f64, t19100: f64, t25590: f64, t25601: f64, t25609: f64, t30569: f64, t30572: f64, t30592: f64, t30595: f64, t30599: f64, t30603: f64) -> (f64, f64, f64, f64) {
    let t30705 = t30691 + t30704;
    let t30707 = t1203 * t30705 * t1212;
    let t30716 = t5770 * t8378;
    let t30729 = -t13110 - 0.23744444444444444444e-1_f64 * t19100 + 0.11872222222222222222e-1_f64 * t25590 - 0.35616666666666666666e-1_f64 * t25601 + 0.17808333333333333333e-1_f64 * t25609 - 0.19787037037037037037e-1_f64 * t30592 + 0.71233333333333333332e-1_f64 * t30595 - 0.35616666666666666666e-1_f64 * t30569 - 0.10685e0_f64 * t30599 + 0.10685e0_f64 * t30572 - 0.17808333333333333333e-1_f64 * t30603;
    (t30705, t30707, t30716, t30729)
}
