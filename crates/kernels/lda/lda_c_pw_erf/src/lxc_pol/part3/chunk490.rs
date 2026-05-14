//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 490/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk490<F: Float>(t2137: F, t548: F, t1475: F, t825: F, t571: F, t1449: F, t798: F, t519: F, t518: F, t821: F) -> (F, F, F, F, F, F) {
    let t2138 = t548 * t2137;
    let t2139 = 8.0 / 45.0 * t2138;
    let t2140 = t1475 * t825;
    let t2141 = t571 * t2140;
    let t2142 = 8.0 / 135.0 * t2141;
    let t2143 = t1449 * t798;
    let t2144 = t519 * t2143;
    let t2145 = 8.0 / 135.0 * t2144;
    let t2146 = t821 * t518;
    (t2139, t2140, t2142, t2143, t2145, t2146)
}
