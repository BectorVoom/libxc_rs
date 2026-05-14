//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1331/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1331<F: Float>(t18231: F, t18234: F, t18238: F, t18241: F, t18242: F, t18243: F, t18244: F, t18247: F, t18250: F, t18252: F, t18257: F, t18259: F, t18261: F, t18263: F, t18267: F, t18269: F, t18271: F) -> (F,) {
    let t19298 = t18231 - t18234 - t18238 + t18241 + t18242 - t18243 - t18244 + t18247 + t18250 - t18252 + t18257 + t18259 + t18261 + t18263 - t18267 + t18269 + t18271;
    (t19298,)
}
