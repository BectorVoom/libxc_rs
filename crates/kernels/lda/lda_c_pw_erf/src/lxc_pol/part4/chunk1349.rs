//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1349/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1349<F: Float>(t2619: F, t474: F, t426: F, t127: F, t1568: F, t1664: F, t19517: F, t19519: F, t19521: F, t19523: F, t19526: F, t19529: F, t19533: F, t19536: F, t2594: F, t2610: F, t3296: F, t7102: F, t8899: F, t9037: F) -> (F, F) {
    let t19539 = t474 * t2619;
    let t19540 = t426 * t19539;
    let t19542 = 5.87616 * t127 * t7102 * t1568 + 176.2848 * t127 * t9037 * t2594 * t1664 - 29.3808 * t127 * t3296 * t2610 * t1664 + t19517 - t19519 - 1.95872 * t8899 + t19521 + 1.95872 * t19523 - 0.48968 * t19526 - t426 * t19529 / 2.0 + 2.0 / 3.0 * t19533 + 3.0 * t426 * t19536 - 2.0 / 9.0 * t19540;
    (t19539, t19542)
}
