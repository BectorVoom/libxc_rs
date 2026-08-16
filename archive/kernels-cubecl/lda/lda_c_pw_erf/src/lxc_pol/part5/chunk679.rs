//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 679/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk679<F: Float>(t171: F, t6039: F, t169: F, t2364: F, t632: F, t2357: F, t462: F, t2700: F, t2703: F, t2709: F, t2712: F, t2739: F, t4395: F, t5969: F, t5970: F, t5971: F, t5972: F, t5973: F, t5974: F, t5975: F, t5976: F, t5977: F, t5978: F) -> (F, F, F, F) {
    let t6040 = t171 * t6039;
    let t6046 = t169 * t2364 * t632;
    let t6052 = t462 * t2357;
    let t6054 = -t5969 + t2700 + t2703 + t5970 - t2709 - t2712 + t5971 - t5972 - t5973 - t4395 - t2739 - t5974 + t5975 - t5976 + t5977 + t5978;
    (t6040, t6046, t6052, t6054)
}
