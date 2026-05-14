//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1348/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1348<F: Float>(t7158: F, t925: F, t7161: F, t8902: F, t1686: F, t2624: F, t933: F, t2627: F, t10: F, t128: F, t19097: F, t2615: F, t474: F, t426: F, t18826: F, t436: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19516 = t7158 * t925;
    let t19517 = 1.2991222222222223 * t19516;
    let t19518 = t7161 * t925;
    let t19519 = 0.6495611111111111 * t19518;
    let t19521 = 3.8973666666666666 * t8902;
    let t19523 = t1686 * t2624 * t933;
    let t19526 = t1686 * t2627 * t933;
    let t19529 = t10 * t128 * t19097;
    let t19532 = t474 * t2615;
    let t19533 = t426 * t19532;
    let t19536 = t10 * t436 * t18826;
    (t19517, t19519, t19521, t19523, t19526, t19529, t19532, t19533, t19536)
}
