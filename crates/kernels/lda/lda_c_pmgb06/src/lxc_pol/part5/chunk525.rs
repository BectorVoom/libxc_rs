//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 525/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk525<F: Float>(t1550: F, t1557: F, t2350: F, t2531: F, t2534: F, t2535: F, t2536: F, t2557: F, t2565: F, t2567: F, t2586: F, t1645: F, t2356: F, t2594: F, t2596: F, t2598: F, t2603: F, t2608: F, t2627: F, t2629: F, t2633: F, t2652: F, t2656: F) -> (F, F) {
    let t2683 = F::cast_from(0.21642082724729686_f64) * t2350 + t2531 + t2534 - t2535 - t2536 - t1550 - t1557 + t2557 + t2565 + t2567 + t2586;
    let t2685 = t2594 + t2596 - t2598 + t2603 + t2608 - t2627 - t2629 - t2633 - t2652 - t2656 + t1645 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2356;
    (t2683, t2685)
}
