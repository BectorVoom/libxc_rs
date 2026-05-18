//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 961/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk961<F: Float>(t12814: F, t10467: F, t1996: F, t519: F, t10463: F, t1972: F, t10313: F, t1967: F, t197: F, t518: F, t5210: F, t1124: F, t213: F) -> (F, F, F, F, F, F) {
    let t12815 = F::new(8.0) / F::new(135.0) * t12814;
    let t12838 = t519 * t10467 * t1996;
    let t12839 = F::new(8.0) / F::new(135.0) * t12838;
    let t12862 = t519 * t10463 * t1972;
    let t12863 = F::new(16.0) / F::new(135.0) * t12862;
    let t12869 = t519 * t10313 * t197 * t1967;
    let t12870 = F::new(8.0) / F::new(81.0) * t12869;
    let t12874 = t5210 * t518;
    let t12916 = t1124 * t213;
    (t12815, t12839, t12863, t12870, t12874, t12916)
}
