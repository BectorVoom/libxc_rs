//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 962/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk962(t11335: f64, t5770: f64, t8228: f64, t5783: f64, t2217: f64, t360: f64, t410: f64, t365: f64, t5740: f64, t5772: f64, t11334: f64, t5756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11336 = 8.769075_f64 * t11335;
    let t11341 = t5770 * t8228;
    let t11343 = t5783 * t8228;
    let t11344 = 2.923025_f64 * t11343;
    let t11354 = t360 * t410 * t2217;
    let t11355 = 2.0_f64 * t11354;
    let t11357 = t365 * t5740 * t5772;
    let t11364 = t365 * t5756 * t11334;
    (t11336, t11341, t11344, t11355, t11357, t11364)
}
