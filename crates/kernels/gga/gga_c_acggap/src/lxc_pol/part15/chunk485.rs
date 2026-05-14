//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 485/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk485<F: Float>(t2671: F, t2674: F, t2677: F, t2679: F, t2683: F, t2685: F, t2687: F, t2690: F, t219: F, t201: F, t132: F, t686: F, t123: F, t265: F, t200: F, t220: F, t721: F) -> (F, F, F, F) {
    let t2692 = -0.25319e1 * t2671 + 0.16879333333333333333e1 * t2674 - 0.19692555555555555555e1 * t2677 - 0.93011851851851851854e0 * t2679 + 0.13651666666666666667e0 * t2683 - 0.27303333333333333333e0 * t2685 - 0.3185388888888888889e0 * t2687 - 0.36514074074074074075e0 * t2690;
    let t2693 = t2692 * t219;
    let t2694 = t201 * t2693;
    let t2695 = 1.0 * t2694;
    let t2696 = t132 * t686;
    let t2700 = t123 * t265;
    let t2707 = t123 * t200;
    let t2709 = t721 * t2707 * t220;
    (t2695, t2696, t2700, t2709)
}
