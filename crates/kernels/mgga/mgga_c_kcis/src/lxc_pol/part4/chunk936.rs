//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 936/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk936<F: Float>(t12844: F, t4441: F, t4439: F, t4425: F, t4446: F, t1599: F, t1590: F, t609: F, t4313: F, t622: F, t1588: F, t4413: F, t12305: F, t1628: F, t4473: F, t1625: F, t4479: F) -> (F, F, F, F, F, F, F, F) {
    let t12845 = t12844 * t4441;
    let t12846 = t4439 * t12845;
    let t12848 = t4425 * t4446;
    let t12849 = t1599 * t12848;
    let t12856 = t1590 * t1590;
    let t12857 = 1.0 / t12856;
    let t12858 = t609 * t12857;
    let t12861 = 1.0 / t4313 / t622;
    let t12890 = t1588 * t4413;
    let t12915 = 0.51588271604938271604e-3 * t12305;
    let t12930 = t4473 * t1628;
    let t12933 = t1625 * t4479;
    (t12846, t12849, t12858, t12861, t12890, t12915, t12930, t12933)
}
