//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 989/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk989<F: Float>(t11543: F, t11597: F, t491: F, t1235: F, t3481: F, t1239: F, t68: F, t1251: F, t3599: F, t225: F, t3484: F, t3493: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11598 = t11543 + t11597;
    let t11599 = t11598 * t491;
    let t11601 = t3481 * t1235;
    let t11604 = t1239 * t1239;
    let t11605 = F::cast_from(1.0_f64) / t11604;
    let t11606 = t68 * t11605;
    let t11607 = t3599 * t1251;
    let t11608 = t11606 * t11607;
    let t11613 = t3484 * t225;
    let t11616 = t11598 * t225;
    let t11620 = t1235 * t3493;
    (t11598, t11599, t11601, t11604, t11605, t11606, t11607, t11608, t11613, t11616, t11620)
}
