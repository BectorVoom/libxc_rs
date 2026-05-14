//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 951/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk951<F: Float>(t11349: F, t11360: F, t20049: F, t20054: F, t20055: F, t8221: F, t8224: F, t8238: F, t8244: F, t8248: F, t8260: F, t8263: F, t8266: F, t8271: F, t8274: F, t8277: F) -> (F,) {
    let t20191 = -t20049 - t11349 - t8221 + t8224 + t8238 - t8244 - t8248 + t8260 + t11360 + t20054 + t8263 - t8266 - t20055 + t8271 + t8274 - t8277;
    (t20191,)
}
