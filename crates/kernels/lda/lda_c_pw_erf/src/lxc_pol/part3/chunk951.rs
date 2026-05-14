//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 951/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk951<F: Float>(t504: F, t529: F, t2176: F, t519: F, t542: F, t1460: F, t348: F, t5255: F, t10474: F, t2183: F, t1325: F, t494: F, t523: F, t798: F, t12578: F, t12580: F, t12582: F, t12587: F, t12589: F, t12591: F, t12595: F, t12599: F) -> (F, F, F, F, F, F) {
    let t12600 = t529 * t504;
    let t12604 = 8.0 / 15.0 * t519 * t2176 * t12600 * t542;
    let t12608 = 8.0 / 9.0 * t519 * t5255 * t1460 * t348;
    let t12610 = 4.0 / 5.0 * t10474 * t2183;
    let t12614 = 16.0 / 15.0 * t1325 * t2176 * t523 * t494;
    let t12615 = t2176 * t798;
    let t12616 = t519 * t12615;
    let t12617 = 32.0 / 1215.0 * t12616;
    let t12618 = -t12578 - t12580 - t12582 - t12587 + t12589 + t12591 - t12595 - t12599 - t12604 + t12608 - t12610 + t12614 + t12617;
    (t12604, t12608, t12610, t12614, t12617, t12618)
}
