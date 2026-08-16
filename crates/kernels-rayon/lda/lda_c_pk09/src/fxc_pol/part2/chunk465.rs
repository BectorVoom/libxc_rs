//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 465/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk465(t2488: f64, t334: f64, t1440: f64, t1442: f64, t2502: f64, t2505: f64, t1439: f64, t1449: f64, t2474: f64, t49: f64, t285: f64, t1248: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2525 = t2488 * t334;
    let t2529 = t1440 - 1.5625_f64 * t2502 + t1442 + 1.5625_f64 * t2505;
    let t2530 = t1439 * t2529;
    let t2531 = t2530 * t1449;
    let t2540 = t49 * t2474;
    let t2541 = t285 * t2540;
    let t2542 = t1248 * t2541;
    (t2525, t2529, t2530, t2531, t2540, t2542)
}
