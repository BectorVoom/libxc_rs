//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 575/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk575<F: Float>(t2623: F, t518: F, t166: F, t161: F, t831: F, t853: F, t2106: F, t822: F, t137: F, t132: F, t1619: F, t2570: F, t2574: F, t473: F, t2578: F, t103: F, t1607: F, t1614: F, t1856: F, t2052: F, t2572: F, t2576: F, t2580: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2624 = t518 * t2623;
    let t2625 = t166 * t2624;
    let t2627 = t161 * t2625 / 30.0;
    let t2629 = t831 * t853 / 15.0;
    let t2630 = t2106 * t822;
    let t2631 = t137 * t2630;
    let t2633 = t132 * t2631 / 15.0;
    let t2639 = t1619 * t2570;
    let t2642 = t473 * t2574;
    let t2645 = t473 * t2578;
    let t2648 = t1607 + 0.023994444444444443 * t1856 - 0.023994444444444443 * t2572 + 0.07198333333333333 * t2576 - 0.035991666666666665 * t2580 + t1614 + 0.008888888888888889 * t2052 - 0.0022222222222222222 * t103 * t2639 + 0.013333333333333334 * t103 * t2642 - 0.006666666666666667 * t103 * t2645;
    (t2624, t2625, t2627, t2629, t2630, t2631, t2633, t2639, t2642, t2645, t2648)
}
