//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1048/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1048<F: Float>(t462: F, t6080: F, t169: F, t18784: F, t242: F, t6035: F, t632: F, t6040: F, t1143: F, t2364: F, t1085: F, t2343: F, t4: F) -> (F, F, F, F, F, F) {
    let t18920 = t462 * t6080;
    let t18923 = t169 * t18784 * t242;
    let t18934 = t169 * t6035 * t632;
    let t18942 = t169 * t6040 * t632;
    let t18945 = t169 * t2364 * t1143;
    let t18965 = t2343 * t4 * t1085;
    (t18920, t18923, t18934, t18942, t18945, t18965)
}
