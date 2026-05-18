//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 991/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk991<F: Float>(t2783: F, t872: F, t1187: F, t2824: F, t483: F, t780: F, t1738: F, t2310: F, t1191: F, t169: F, t1891: F, t301: F) -> (F, F, F, F) {
    let t11557 = t2783 * t872;
    let t11561 = t2824 * t780 * t483 * t1187;
    let t11563 = t1738 * t2310;
    let t11567 = t169 * t1191 * t1891 * t301;
    (t11557, t11561, t11563, t11567)
}
