//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1271/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1271<F: Float>(t15434: F, t15435: F, t15436: F, t15437: F, t15438: F, t15439: F, t15440: F, t8221: F, t8224: F, t8238: F, t8244: F, t8248: F, t8260: F, t8263: F, t8266: F, t8271: F, t8274: F) -> (F,) {
    let t18959 = -t8221 + t8224 + t8238 - t8244 - t8248 - t15434 + t15435 - t15436 - t15437 + t8260 + t15438 + t15439 + t8263 - t8266 - t15440 + t8271 + t8274;
    (t18959,)
}
