//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 669/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk669(t285: f64, t6067: f64, t2395: f64, t686: f64, t248: f64, t2396: f64, t638: f64, t643: f64, t27: f64, t693: f64, t3662: f64, t3672: f64, t3678: f64, t3700: f64, t4483: f64, t4485: f64, t4520: f64, t4522: f64, t4525: f64, t4531: f64, t6038: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6068 = t6067 * t285;
    let t6070 = t2395 * t686;
    let t6071 = t248 * t6070;
    let t6072 = t638 * t2396;
    let t6074 = t643 * t2396;
    let t6078 = t2395 * t27;
    let t6079 = t6078 * t693;
    let t6081 = -0.5848223622634646_f64 * t6038 + t4483 - t4485 - 24.0_f64 * t4520 + 40.0_f64 * t4522 + t248 * t6068 + t6071 + 4.0_f64 * t6072 - 4.0_f64 * t6074 + 2.0_f64 * t4525 + 0.00024415263074675396_f64 * t3662 + t3672 - t3678 + t3700 - 0.00018311447306006544_f64 * t6079 - t4531;
    (t6068, t6070, t6071, t6072, t6074, t6078, t6079, t6081)
}
