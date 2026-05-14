//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 829/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk829<F: Float>(t11316: F, t5783: F, t5770: F, t2217: F, t360: F, t410: F, t1271: F, t2233: F, t955: F, t1238: F, t2210: F, t1830: F, t2226: F, t2186: F, t1180: F, t776: F) -> (F, F, F, F, F, F, F, F) {
    let t11317 = t5783 * t11316;
    let t11318 = 3.8973666666666666 * t11317;
    let t11322 = t5770 * t11316;
    let t11323 = 11.75232 * t11322;
    let t11354 = t360 * t410 * t2217;
    let t11355 = 2.0 * t11354;
    let t11373 = t1271 * t2233 * t955;
    let t11374 = 1.46904 * t11373;
    let t11379 = t1238 * t2210 * t955;
    let t11380 = 0.9743416666666667 * t11379;
    let t11388 = t2226 * t1830;
    let t11390 = t2186 * t1830;
    let t11392 = t1180 * t776;
    (t11318, t11323, t11355, t11374, t11380, t11388, t11390, t11392)
}
