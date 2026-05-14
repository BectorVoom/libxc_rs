//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 861/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk861<F: Float>(t12814: F, t10467: F, t1996: F, t519: F, t10463: F, t1972: F, t10313: F, t1967: F, t197: F, t518: F, t5210: F, t1124: F, t213: F, t4489: F, t784: F, t34: F, t3966: F) -> (F, F, F, F, F, F, F, F) {
    let t12815 = 8.0 / 135.0 * t12814;
    let t12838 = t519 * t10467 * t1996;
    let t12839 = 8.0 / 135.0 * t12838;
    let t12862 = t519 * t10463 * t1972;
    let t12863 = 16.0 / 135.0 * t12862;
    let t12869 = t519 * t10313 * t197 * t1967;
    let t12870 = 8.0 / 81.0 * t12869;
    let t12874 = t5210 * t518;
    let t12916 = t1124 * t213;
    let t12956 = t4489 * t784;
    let t12963 = t3966 * t34;
    (t12815, t12839, t12863, t12870, t12874, t12916, t12956, t12963)
}
