//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 492/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk492<F: Float>(t34: F, t575: F, t2151: F, t571: F, t581: F, t833: F, t549: F, t1466: F, t1318: F, t1401: F, t593: F, t529: F, t784: F, t542: F, t1440: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2152 = t575 * t34;
    let t2153 = t2151 * t2152;
    let t2155 = 8.0 / 45.0 * t571 * t2153;
    let t2156 = t581 * t833;
    let t2157 = t2156 * t549;
    let t2158 = t1466 * t2157;
    let t2160 = 4.0 / 15.0 * t1318 * t2158;
    let t2161 = t1401 * t833;
    let t2162 = t2161 * t593;
    let t2163 = t1466 * t2162;
    let t2165 = 4.0 / 15.0 * t571 * t2163;
    let t2166 = t529 * t784;
    let t2167 = t2166 * t542;
    let t2168 = t1440 * t2167;
    (t2152, t2153, t2155, t2156, t2157, t2158, t2160, t2161, t2162, t2163, t2165, t2166, t2167, t2168)
}
