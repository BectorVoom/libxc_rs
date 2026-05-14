//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 757/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk757<F: Float>(t1458: F, t473: F, t197: F, t4620: F, t519: F, t1995: F, t945: F, t1313: F, t1245: F, t784: F, t940: F, t1991: F, t1325: F, t1401: F, t1466: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5255 = t473 * t1458;
    let t5256 = t5255 * t197;
    let t5257 = t5256 * t4620;
    let t5259 = 16.0 / 27.0 * t519 * t5257;
    let t5260 = t1995 * t945;
    let t5261 = t1313 * t5260;
    let t5263 = 4.0 / 45.0 * t519 * t5261;
    let t5264 = t784 * t1245;
    let t5265 = t5264 * t940;
    let t5266 = t1991 * t5265;
    let t5268 = 8.0 / 27.0 * t1325 * t5266;
    let t5269 = t1466 * t1401;
    (t5255, t5256, t5257, t5259, t5260, t5261, t5263, t5265, t5266, t5268, t5269)
}
