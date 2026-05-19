//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 495/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk495<F: Float>(t2671: F, t2674: F, t2677: F, t2679: F, t2683: F, t2685: F, t2687: F, t2690: F, t219: F, t201: F, t132: F, t686: F) -> (F, F) {
    let t2692 = -F::new(0.25319e1) * t2671 + F::cast_from(0.16879333333333333333e1_f64) * t2674 - F::cast_from(0.19692555555555555555e1_f64) * t2677 - F::cast_from(0.93011851851851851854e0_f64) * t2679 + F::cast_from(0.13651666666666666667e0_f64) * t2683 - F::cast_from(0.27303333333333333333e0_f64) * t2685 - F::cast_from(0.3185388888888888889e0_f64) * t2687 - F::cast_from(0.36514074074074074075e0_f64) * t2690;
    let t2693 = t2692 * t219;
    let t2694 = t201 * t2693;
    let t2695 = F::new(1.0) * t2694;
    let t2696 = t132 * t686;
    (t2695, t2696)
}
