//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 571/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk571<F: Float>(t1755: F, t72: F, t1526: F, t1527: F, t1538: F, t1565: F, t1761: F, t3088: F, t342: F, t343: F, t7704: F, t7707: F, t7710: F, t7713: F, t7721: F, t7725: F) -> (F,) {
    let t7729 = t72 * t1755;
    let t7733 = t1538 + t1761 + t7704 - t7707 / 18.0 - t7710 / 6.0 - t1526 * t3088 * t7713 / 9.0 - t1526 * t1527 * t1565 / 6.0 + t1526 * t1527 * t7721 / 6.0 - t1526 * t1527 * t7725 / 12.0 - t342 * t343 * t7729 / 4.0;
    (t7733,)
}
