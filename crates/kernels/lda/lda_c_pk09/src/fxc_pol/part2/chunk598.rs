//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 598/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk598<F: Float>(t5308: F, t5829: F, t327: F, t5009: F, t309: F, t310: F, t4993: F, t1240: F, t1434: F, t1637: F, t318: F, t319: F, t5759: F, t1634: F, t5569: F, t1336: F, t1580: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5830 = t5829 * t5308;
    let t5832 = t327 * t5009;
    let t5834 = t309 * t310 * t4993;
    let t5836 = t5832 * t5834 / 3.0;
    let t5838 = t309 * t1434 * t1240;
    let t5840 = t1637 * t5838 / 9.0;
    let t5845 = t318 * t5009;
    let t5847 = t5845 * t5834 / 3.0;
    let t5854 = t319 * t5759;
    let t5856 = t1634 * t5569;
    let t5864 = t1580 * t1336;
    (t5830, t5834, t5836, t5838, t5840, t5847, t5854, t5856, t5864)
}
