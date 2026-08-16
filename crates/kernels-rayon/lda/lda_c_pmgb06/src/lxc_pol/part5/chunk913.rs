//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 913/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk913(t11304: f64, t11303: f64, t1238: f64, t776: f64, t30: f64, t342: f64, t410: f64, t5783: f64, t5770: f64, t2217: f64, t360: f64, t1271: f64, t2233: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11305 = 5.87616_f64 * t11304;
    let t11310 = t1238 * t776 * t11303;
    let t11311 = 1.9486833333333333_f64 * t11310;
    let t11316 = t30 * t410 * t342;
    let t11317 = t5783 * t11316;
    let t11318 = 3.8973666666666666_f64 * t11317;
    let t11322 = t5770 * t11316;
    let t11323 = 11.75232_f64 * t11322;
    let t11354 = t360 * t410 * t2217;
    let t11355 = 2.0_f64 * t11354;
    let t11373 = t1271 * t2233 * t955;
    (t11305, t11311, t11318, t11323, t11355, t11373)
}
