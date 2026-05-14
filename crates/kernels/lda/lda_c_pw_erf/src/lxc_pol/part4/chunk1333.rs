//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1333/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1333<F: Float>(t18318: F, t18320: F, t18324: F, t18328: F, t18330: F, t18333: F, t18336: F, t18339: F, t18341: F, t18346: F, t18350: F, t18352: F, t18354: F, t18356: F, t18358: F, t18359: F, t18361: F) -> (F,) {
    let t19301 = t18318 - t18320 - t18324 - t18328 - t18330 - t18333 - t18336 - t18339 + t18341 + t18346 + t18350 - t18352 + t18354 + t18356 + t18358 - t18359 - t18361;
    (t19301,)
}
