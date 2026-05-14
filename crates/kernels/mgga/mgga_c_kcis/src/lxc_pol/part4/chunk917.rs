//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 917/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk917<F: Float>(t1311: F, t3898: F, t3897: F, t465: F, t455: F, t11407: F, t11481: F, t127: F, t1392: F, t368: F, t456: F, t518: F, t531: F, t10338: F, t1474: F, t3255: F, t3777: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11576 = t1311 * t3898;
    let t11580 = 1.0 / t3897 / t465;
    let t11581 = t455 * t11580;
    let t11608 = 0.93011851851851851854e0 * t11407;
    let t11609 = 0.36514074074074074075e0 * t11481;
    let t11632 = t127 * t368 * t1392;
    let t11633 = t456 * t518;
    let t11634 = t11633 * t531;
    let t11640 = t10338 * t1474;
    let t11642 = t3255 * t3777;
    (t11576, t11581, t11608, t11609, t11632, t11633, t11634, t11640, t11642)
}
