//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 667/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk667<F: Float>(t1319: F, t6413: F, t571: F, t2325: F, t3518: F, t348: F, t5250: F, t519: F, t1966: F, t34: F, t5256: F, t2471: F, t504: F, t3806: F, t784: F, t806: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6414 = t1319 * t6413;
    let t6416 = 8.0 / 15.0 * t571 * t6414;
    let t6417 = t3518 * t2325;
    let t6418 = t6417 * t348;
    let t6419 = t5250 * t6418;
    let t6421 = 32.0 / 81.0 * t519 * t6419;
    let t6422 = t1966 * t34;
    let t6423 = t5256 * t6422;
    let t6425 = 16.0 / 27.0 * t519 * t6423;
    let t6426 = t2471 * t504;
    let t6427 = t6426 * t348;
    let t6428 = t3806 * t6427;
    let t6430 = 8.0 / 45.0 * t519 * t6428;
    let t6431 = t784 * t806;
    (t6414, t6416, t6417, t6418, t6419, t6421, t6422, t6423, t6425, t6426, t6427, t6428, t6430, t6431)
}
