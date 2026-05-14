//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 505/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk505<F: Float>(t211: F, t2123: F, t1521: F, t1531: F, t2044: F, t2046: F, t2069: F, t2074: F, t2078: F, t2080: F, t2102: F, t2106: F, t2108: F, t2110: F, t2112: F, t2116: F, t2118: F, t2122: F) -> (F, F, F) {
    let t2124 = t211 * t2123;
    let t2125 = 4.0 / 45.0 * t2124;
    let t2126 = -t1521 + t2044 - t1531 - t2046 - t2069 - t2074 + t2078 - t2080 - t2102 + t2106 + t2108 - t2110 - t2112 + t2116 + t2118 + t2122 - t2125;
    (t2124, t2125, t2126)
}
