//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1076/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1076<F: Float>(t471: F, t476: F, t2006: F, t2811: F, t7292: F, t2016: F, t309: F, t454: F, t2812: F, t7300: F, t2042: F, t1905: F, t7704: F) -> (F, F, F, F, F, F) {
    let t11711 = t471 * t476;
    let t11714 = t2811 * t2006;
    let t11715 = t11714 * t7292;
    let t11717 = t309 * t454 * t2016;
    let t11720 = t2812 * t7300;
    let t11721 = t11720 * t2042;
    let t11723 = t2812 * t7292;
    let t11733 = t309 * t1905 * t7704;
    (t11711, t11715, t11717, t11721, t11723, t11733)
}
