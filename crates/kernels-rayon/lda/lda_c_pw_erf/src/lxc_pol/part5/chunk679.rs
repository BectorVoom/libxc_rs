//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 679/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk679(t171: f64, t6039: f64, t169: f64, t2364: f64, t632: f64, t2357: f64, t462: f64, t2700: f64, t2703: f64, t2709: f64, t2712: f64, t2739: f64, t4395: f64, t5969: f64, t5970: f64, t5971: f64, t5972: f64, t5973: f64, t5974: f64, t5975: f64, t5976: f64, t5977: f64, t5978: f64) -> (f64, f64, f64, f64) {
    let t6040 = t171 * t6039;
    let t6046 = t169 * t2364 * t632;
    let t6052 = t462 * t2357;
    let t6054 = -t5969 + t2700 + t2703 + t5970 - t2709 - t2712 + t5971 - t5972 - t5973 - t4395 - t2739 - t5974 + t5975 - t5976 + t5977 + t5978;
    (t6040, t6046, t6052, t6054)
}
