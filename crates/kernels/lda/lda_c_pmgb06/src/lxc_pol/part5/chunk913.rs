//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 913/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk913<F: Float>(t11304: F, t11303: F, t1238: F, t776: F, t30: F, t342: F, t410: F, t5783: F, t5770: F, t2217: F, t360: F, t1271: F, t2233: F, t955: F) -> (F, F, F, F, F, F) {
    let t11305 = F::new(5.87616) * t11304;
    let t11310 = t1238 * t776 * t11303;
    let t11311 = F::new(1.9486833333333333) * t11310;
    let t11316 = t30 * t410 * t342;
    let t11317 = t5783 * t11316;
    let t11318 = F::new(3.8973666666666666) * t11317;
    let t11322 = t5770 * t11316;
    let t11323 = F::new(11.75232) * t11322;
    let t11354 = t360 * t410 * t2217;
    let t11355 = F::new(2.0) * t11354;
    let t11373 = t1271 * t2233 * t955;
    (t11305, t11311, t11318, t11323, t11355, t11373)
}
