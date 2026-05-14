//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1060/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1060<F: Float>(t11348: F, t8249: F, t8251: F, t11355: F, t8255: F, t11359: F, t11362: F, t8267: F, t8278: F, t8221: F, t8224: F, t8238: F, t8244: F, t8248: F, t8260: F, t8263: F, t8266: F, t8271: F, t8274: F, t8277: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15433 = 8.0 * t11348;
    let t15434 = 0.5848223397455204 * t8249;
    let t15435 = 1.169644679491041 * t8251;
    let t15436 = 1.169644679491041 * t11355;
    let t15437 = 8.0 * t8255;
    let t15438 = 4.0 * t11359;
    let t15439 = 2.0 * t11362;
    let t15440 = 0.043374323531126094 * t8267;
    let t15441 = 0.06506148529668915 * t8278;
    let t15442 = t15433 - t8221 + t8224 + t8238 - t8244 - t8248 - t15434 + t15435 - t15436 - t15437 + t8260 + t15438 + t15439 + t8263 - t8266 - t15440 + t8271 + t8274 - t8277 - t15441;
    (t15433, t15434, t15435, t15436, t15437, t15438, t15439, t15440, t15441, t15442)
}
