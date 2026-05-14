//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1329/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1329<F: Float>(t15107: F, t15109: F, t15111: F, t18159: F, t18161: F, t18164: F, t18168: F, t18170: F, t18172: F, t18174: F, t18177: F, t18178: F, t18180: F, t18183: F, t18187: F, t18191: F, t18193: F) -> (F,) {
    let t19294 = -t18159 - t18161 - t18164 - t18168 - t18170 - t18172 - t18174 - 8.0 / 27.0 * t15107 - 4.0 / 9.0 * t15109 + 16.0 / 81.0 * t15111 - t18177 + t18178 - t18180 + t18183 - t18187 - t18191 - t18193;
    (t19294,)
}
