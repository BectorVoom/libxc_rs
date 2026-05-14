//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 753/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk753<F: Float>(t496: F, t5215: F, t1245: F, t806: F, t940: F, t3402: F, t519: F, t2000: F, t954: F, t1319: F, t1318: F, t1351: F, t811: F, t951: F, t2017: F, t1972: F, t3859: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5217 = 8.0 / 15.0 * t5215 * t496;
    let t5220 = t806 * t1245;
    let t5221 = t5220 * t940;
    let t5222 = t3402 * t5221;
    let t5224 = 4.0 / 27.0 * t519 * t5222;
    let t5225 = t2000 * t954;
    let t5226 = t1319 * t5225;
    let t5228 = 8.0 / 45.0 * t1318 * t5226;
    let t5229 = t811 * t1351;
    let t5230 = t5229 * t951;
    let t5231 = t2017 * t5230;
    let t5233 = 8.0 / 27.0 * t1318 * t5231;
    let t5234 = t3859 * t1972;
    (t5217, t5221, t5222, t5224, t5225, t5226, t5228, t5230, t5231, t5233, t5234)
}
