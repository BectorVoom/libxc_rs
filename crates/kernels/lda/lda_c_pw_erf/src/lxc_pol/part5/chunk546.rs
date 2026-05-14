//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 546/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk546<F: Float>(t3259: F, t3267: F, t1657: F, t3216: F, t1653: F, t2061: F, t1953: F, t432: F, t416: F, t1124: F, t118: F, t119: F, t120: F, t1687: F, t435: F, t96: F) -> (F, F, F, F, F, F, F, F) {
    let t3268 = t3267 * t3259;
    let t3276 = t1657 * t3216;
    let t3280 = 1.2991222222222223 * t1653 * t2061;
    let t3282 = 0.7617244444444444 * t432 * t1953;
    let t3284 = 1.5156425925925925 * t416 * t1953;
    let t3288 = 7.0 / 27.0 * t118 * t119 * t1124 * t120;
    let t3290 = 0.6529066666666666 * t1687 * t2061;
    let t3296 = 1.0 / t435 / t96;
    (t3268, t3276, t3280, t3282, t3284, t3288, t3290, t3296)
}
