//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1241/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1241<F: Float>(t18542: F, t18566: F, t38: F, t56: F, t14816: F, t64: F, t365: F, t5772: F, t6996: F, t2703: F, t348: F, t110: F, t2209: F, t30: F, t5783: F, t11237: F, t1234: F, t1282: F, t18503: F, t18507: F, t18518: F, t2448: F, t2695: F, t3615: F, t370: F, t63: F, t8245: F) -> (F, F, F, F, F, F, F) {
    let t18568 = t18542 / 2.0 + t18566 / 2.0;
    let t18571 = 2.923025 * t38 * t56 * t18568;
    let t18580 = 11.6921 * t38 * t64 * t14816;
    let t18582 = t365 * t6996 * t5772;
    let t18585 = t348 * t2703 * t5772;
    let t18586 = 5.84605 * t18585;
    let t18588 = t30 * t110 * t2209;
    let t18589 = t5783 * t18588;
    let t18590 = 3.8973666666666666 * t18589;
    let t18591 = t18503 - 1.95872 * t11237 - t18507 + 176.2848 * t63 * t8245 * t2695 * t1234 - 29.3808 * t63 * t3615 * t2448 * t1234 + t18518 - t18571 - 1.46904 * t63 * t370 * t18568 + 11.75232 * t63 * t1282 * t14816 + t18580 - 5.87616 * t18582 + t18586 - t18590;
    (t18568, t18571, t18580, t18586, t18588, t18590, t18591)
}
