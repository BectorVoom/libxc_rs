//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1070/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1070<F: Float>(t30551: F, t3725: F, t1212: F, t30705: F, t12888: F, t14831: F, t19100: F, t25590: F, t25601: F, t25609: F, t30569: F, t30572: F, t30592: F, t30595: F, t30599: F, t30603: F) -> (F, F, F, F) {
    let t31581 = t30551 * t3725;
    let t31584 = t30705 * t1212;
    let t31587 = t30551 * t12888;
    let t31603 = -t14831 - F::cast_from(0.2283111111111111111e-1_f64) * t19100 + F::cast_from(0.11415555555555555555e-1_f64) * t25590 - F::cast_from(0.34246666666666666665e-1_f64) * t25601 + F::cast_from(0.17123333333333333333e-1_f64) * t25609 - F::cast_from(0.19025925925925925925e-1_f64) * t30592 + F::cast_from(0.68493333333333333331e-1_f64) * t30595 - F::cast_from(0.34246666666666666665e-1_f64) * t30569 - F::new(0.10274e0) * t30599 + F::new(0.10274e0) * t30572 - F::cast_from(0.17123333333333333333e-1_f64) * t30603;
    (t31581, t31584, t31587, t31603)
}
