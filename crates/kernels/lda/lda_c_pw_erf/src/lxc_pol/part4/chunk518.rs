//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 518/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk518<F: Float>(t34: F, t523: F, t2176: F, t519: F, t529: F, t806: F, t494: F, t1440: F, t1325: F, t1390: F, t542: F, t581: F, t811: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2177 = t523 * t34;
    let t2178 = t2176 * t2177;
    let t2180 = 8.0 / 45.0 * t519 * t2178;
    let t2181 = t529 * t806;
    let t2182 = t2181 * t494;
    let t2183 = t1440 * t2182;
    let t2185 = 4.0 / 15.0 * t1325 * t2183;
    let t2186 = t1390 * t806;
    let t2187 = t2186 * t542;
    let t2188 = t1440 * t2187;
    let t2190 = 4.0 / 15.0 * t519 * t2188;
    let t2191 = t581 * t811;
    (t2177, t2178, t2180, t2181, t2182, t2183, t2185, t2186, t2187, t2188, t2190, t2191)
}
