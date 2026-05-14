//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 570/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk570<F: Float>(t7639: F, t2252: F, t342: F, t344: F, t173: F, t422: F, t1526: F, t1529: F, t1533: F, t630: F, t1557: F, t81: F, t1559: F, t1570: F, t1528: F, t1580: F) -> (F, F, F, F, F, F, F, F) {
    let t7640 = 1.0 / t7639;
    let t7704 = t342 * t2252 * t344 / 18.0;
    let t7705 = t173 * t422;
    let t7707 = t1526 * t7705 * t1529;
    let t7710 = t342 * t630 * t1533;
    let t7712 = t81 * t1557;
    let t7713 = t7712 * t1559;
    let t7720 = t81 * t1570;
    let t7721 = t7720 * t1559;
    let t7725 = t1528 * t1580;
    (t7640, t7704, t7705, t7707, t7710, t7713, t7721, t7725)
}
