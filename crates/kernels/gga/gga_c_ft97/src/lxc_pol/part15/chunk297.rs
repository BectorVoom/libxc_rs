//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 297/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk297<F: Float>(t1609: F, t371: F, t409: F, t29: F, t30: F, t25: F) -> (F, F, F, F, F, F, F) {
    let t1610 = t1609 * t1609;
    let t1611 = t1610 * t1610;
    let t1613 = t1611 * t1611;
    let t1614 = t1613 * t1611 * t1609;
    let t1624 = t371 * t409;
    let t1630 = F::new(1.0) / t30 / t29;
    let t1631 = t25 * t1630;
    (t1610, t1611, t1613, t1614, t1624, t1630, t1631)
}
