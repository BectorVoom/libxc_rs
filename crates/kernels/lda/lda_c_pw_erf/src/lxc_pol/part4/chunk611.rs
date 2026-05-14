//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 611/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk611<F: Float>(t364: F, t473: F, t155: F, t988: F, t1010: F, t1953: F, t2061: F, t2717: F, t2720: F, t2723: F, t2728: F, t2730: F, t2732: F, t371: F, t363: F, t987: F) -> (F, F, F, F, F, F) {
    let t3031 = t473 * t364;
    let t3038 = t155 * t988;
    let t3046 = t155 * t1010;
    let t3058 = -4.7063 * t2717 + 3.1375333333333333 * t2720 - 3.6604555555555556 * t2723 - 1.6068111111111112 * t1953 + 0.2805166666666667 * t2728 - 0.5610333333333334 * t2730 - 0.6545388888888889 * t2732 - 0.4630888888888889 * t2061;
    let t3059 = t3058 * t371;
    let t3063 = 1.0 / t987 / t363;
    (t3031, t3038, t3046, t3058, t3059, t3063)
}
