//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 221/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk221<F: Float>(t1614: F, t77: F, t373: F, t1608: F, t51: F, t53: F, t397: F, t371: F, t409: F, t29: F, t30: F, t25: F) -> (F, F, F, F, F, F) {
    let t1615 = t77 * t1614;
    let t1616 = t1615 * t373;
    let t1617 = t1608 * t1616;
    let t1619 = t51 * t53;
    let t1620 = t1619 * t397;
    let t1624 = t371 * t409;
    let t1630 = F::cast_from(1.0_f64) / t30 / t29;
    let t1631 = t25 * t1630;
    (t1616, t1617, t1620, t1624, t1630, t1631)
}
