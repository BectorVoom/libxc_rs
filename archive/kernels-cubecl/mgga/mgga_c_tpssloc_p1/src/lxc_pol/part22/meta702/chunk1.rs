//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2289/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2289<F: Float>(t15486: F, t5024: F, t15590: F, t5018: F, t15507: F, t15548: F, t13969: F, t19057: F, t3506: F, t15438: F, t15569: F, t15608: F) -> (F, F, F, F, F, F) {
    let t66155 = t5024 * t15486;
    let t66159 = t15590 * t5018;
    let t66165 = t15507 * t15548;
    let t66241 = t3506 * t13969 * t19057;
    let t66255 = t15438 * t15548;
    let t66268 = t15569 * t15608;
    (t66155, t66159, t66165, t66241, t66255, t66268)
}
