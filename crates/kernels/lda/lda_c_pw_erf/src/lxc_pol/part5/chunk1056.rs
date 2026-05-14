//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1056/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1056<F: Float>(t21967: F, t1449: F, t519: F, t7474: F, t2140: F, t6205: F, t7007: F, t3899: F, t571: F, t7557: F, t13515: F, t13508: F, t13512: F, t13518: F, t21949: F, t21954: F, t21958: F, t21962: F, t21965: F) -> (F, F, F, F, F, F, F) {
    let t21968 = 8.0 / 15.0 * t21967;
    let t21970 = t519 * t1449 * t7474;
    let t21971 = 16.0 / 45.0 * t21970;
    let t21972 = t6205 * t2140;
    let t21973 = 8.0 / 45.0 * t21972;
    let t21974 = t7007 * t2140;
    let t21975 = 16.0 / 45.0 * t21974;
    let t21977 = t571 * t3899 * t7557;
    let t21978 = 8.0 / 15.0 * t21977;
    let t21979 = 16.0 / 135.0 * t13515;
    let t21980 = -t21949 + t21954 - t21958 - t21962 + t21965 - t21968 + t21971 + t21973 + t21975 + t21978 + t13508 - t13512 + t21979 - t13518;
    (t21968, t21971, t21973, t21975, t21978, t21979, t21980)
}
