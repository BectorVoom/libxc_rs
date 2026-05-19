//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1018/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1018<F: Float>(t30691: F, t30704: F, t1203: F, t1212: F, t5770: F, t8378: F, t13110: F, t19100: F, t25590: F, t25601: F, t25609: F, t30569: F, t30572: F, t30592: F, t30595: F, t30599: F, t30603: F) -> (F, F, F, F) {
    let t30705 = t30691 + t30704;
    let t30707 = t1203 * t30705 * t1212;
    let t30716 = t5770 * t8378;
    let t30729 = -t13110 - F::cast_from(0.23744444444444444444e-1_f64) * t19100 + F::cast_from(0.11872222222222222222e-1_f64) * t25590 - F::cast_from(0.35616666666666666666e-1_f64) * t25601 + F::cast_from(0.17808333333333333333e-1_f64) * t25609 - F::cast_from(0.19787037037037037037e-1_f64) * t30592 + F::cast_from(0.71233333333333333332e-1_f64) * t30595 - F::cast_from(0.35616666666666666666e-1_f64) * t30569 - F::new(0.10685e0) * t30599 + F::new(0.10685e0) * t30572 - F::cast_from(0.17808333333333333333e-1_f64) * t30603;
    (t30705, t30707, t30716, t30729)
}
