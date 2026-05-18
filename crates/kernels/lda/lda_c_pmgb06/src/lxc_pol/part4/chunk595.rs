//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 595/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk595<F: Float>(t2623: F, t518: F, t166: F, t161: F, t831: F, t853: F, t2106: F, t822: F, t137: F, t132: F, t1619: F, t2570: F) -> (F, F, F, F, F, F, F, F) {
    let t2624 = t518 * t2623;
    let t2625 = t166 * t2624;
    let t2627 = t161 * t2625 / F::new(30.0);
    let t2629 = t831 * t853 / F::new(15.0);
    let t2630 = t2106 * t822;
    let t2631 = t137 * t2630;
    let t2633 = t132 * t2631 / F::new(15.0);
    let t2639 = t1619 * t2570;
    (t2624, t2625, t2627, t2629, t2630, t2631, t2633, t2639)
}
