//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 601/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk601(t1550: f64, t1557: f64, t2350: f64, t2531: f64, t2534: f64, t2535: f64, t2536: f64, t2557: f64, t2565: f64, t2567: f64, t2586: f64, t1645: f64, t2356: f64, t2594: f64, t2596: f64, t2598: f64, t2603: f64, t2608: f64, t2627: f64, t2629: f64, t2633: f64, t2652: f64, t2656: f64) -> (f64, f64) {
    let t2683 = 0.21642082724729686_f64 * t2350 + t2531 + t2534 - t2535 - t2536 - t1550 - t1557 + t2557 + t2565 + t2567 + t2586;
    let t2685 = t2594 + t2596 - t2598 + t2603 + t2608 - t2627 - t2629 - t2633 - t2652 - t2656 + t1645 + 8.0_f64 / 3.0_f64 * t2356;
    (t2683, t2685)
}
