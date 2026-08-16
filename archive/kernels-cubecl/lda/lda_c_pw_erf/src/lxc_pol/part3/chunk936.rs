//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 936/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk936<F: Float>(t1393: F, t1518: F, t185: F, t3546: F, t514: F, t4039: F, t511: F, t4036: F, t568: F, t1325: F, t3787: F, t3798: F) -> (F, F, F, F, F) {
    let t10422 = t185 * t1518 * t1393;
    let t10425 = t185 * t514 * t3546;
    let t10427 = t511 * t4039;
    let t10429 = t4036 * t568;
    let t10432 = t1325 * t3787 * t3798;
    (t10422, t10425, t10427, t10429, t10432)
}
