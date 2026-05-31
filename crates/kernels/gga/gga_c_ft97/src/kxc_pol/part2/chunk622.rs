//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 622/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk622<F: Float>(t1557: F, t81: F, t1559: F, t1570: F, t1528: F, t1580: F, t1755: F, t72: F, t1526: F, t1527: F, t1538: F, t1565: F, t1761: F, t3088: F, t342: F, t343: F, t7704: F, t7707: F, t7710: F) -> F {
    let t7712 = t81 * t1557;
    let t7713 = t7712 * t1559;
    let t7720 = t81 * t1570;
    let t7721 = t7720 * t1559;
    let t7725 = t1528 * t1580;
    let t7729 = t72 * t1755;
    let t7733 = t1538 + t1761 + t7704 - t7707 / F::cast_from(18.0_f64) - t7710 / F::cast_from(6.0_f64) - t1526 * t3088 * t7713 / F::cast_from(9.0_f64) - t1526 * t1527 * t1565 / F::cast_from(6.0_f64) + t1526 * t1527 * t7721 / F::cast_from(6.0_f64) - t1526 * t1527 * t7725 / F::cast_from(12.0_f64) - t342 * t343 * t7729 / F::cast_from(4.0_f64);
    t7733
}
