//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1008/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1008<F: Float>(t1308: F, t2065: F, t2419: F, t571: F, t1446: F, t7706: F, t2098: F, t2429: F, t3402: F, t519: F, t1325: F, t1991: F, t494: F, t7639: F, t542: F, t7692: F) -> (F, F, F, F, F, F) {
    let t21159 = 4.0 / 15.0 * t571 * t1308 * t2419 * t2065;
    let t21161 = 4.0 / 9.0 * t1446 * t7706;
    let t21165 = 4.0 / 9.0 * t519 * t3402 * t2429 * t2098;
    let t21169 = 16.0 / 9.0 * t1325 * t1991 * t7639 * t494;
    let t21173 = 8.0 / 9.0 * t519 * t3402 * t7639 * t542;
    let t21175 = 4.0 / 15.0 * t1446 * t7692;
    (t21159, t21161, t21165, t21169, t21173, t21175)
}
