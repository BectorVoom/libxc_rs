//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 887/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk887<F: Float>(t10832: F, t4429: F, t1809: F, t2790: F, t169: F, t2817: F, t301: F, t865: F, t10823: F, t10849: F, t11557: F, t11561: F, t11563: F, t11568: F, t11570: F, t11574: F, t1550: F, t1733: F, t1881: F, t2211: F, t2764: F, t2767: F, t2799: F, t4117: F, t4441: F, t5670: F, t777: F, t9127: F) -> (F,) {
    let t11577 = t10832 * t4429;
    let t11588 = t2790 * t1809;
    let t11597 = t169 * t2817 * t865 * t301;
    let t11599 = 0.19513566535229734 * t11557 + 0.0001639671923854359 * t11561 - 0.15965645347006147 * t11563 + t11568 - 9.0 * t2764 * t11570 - 18.0 * t11574 * t2767 - 6.0 * t2764 * t11577 + 6.0 * t4117 * t4441 + 3.0 * t2211 * t10823 + 9.0 * t2211 * t10849 - 3.0 * t1881 * t2799 + 3.0 * t1733 * t11588 - 3.0 * t777 * t9127 + 3.0 * t5670 * t1550 - 0.9247854820715865 * t11597;
    (t11599,)
}
