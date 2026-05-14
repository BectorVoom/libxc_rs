//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 519/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk519<F: Float>(t119: F, t3557: F, t748: F, t827: F, t609: F, t873: F, t96: F, t839: F, t1067: F, t864: F, t3330: F, t3332: F, t3339: F, t3498: F, t980: F, t161: F, t3230: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3558 = t119 * t3557;
    let t3559 = 24.533164868110067 * t3558;
    let t3568 = t748 * t827;
    let t3577 = t96 * t873 * t609;
    let t3578 = t839 * t3577;
    let t3580 = t864 * t1067;
    let t3598 = 2.0 * t3330;
    let t3599 = 8.0 / 3.0 * t3332;
    let t3601 = 4.0 / 9.0 * t3339;
    let t3613 = 6.5831116232644735 * t980 * t3498;
    let t3614 = t161 * t3230;
    (t3558, t3559, t3568, t3578, t3580, t3598, t3599, t3601, t3613, t3614)
}
