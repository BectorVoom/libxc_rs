//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 813/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk813<F: Float>(t2: F, t38482: F, t1788: F, t8282: F, t458: F, t8305: F, t11690: F, t1787: F, t3127: F, t3134: F, t37273: F, t37283: F, t37306: F, t37311: F, t37315: F, t37320: F, t38264: F, t38269: F, t38273: F, t38283: F, t38461: F, t38464: F, t38478: F, t462: F, t8291: F, t8327: F) -> (F,) {
    let t38483 = t38482 * t2;
    let t38490 = t8282 * t1788;
    let t38501 = t458 * t8305;
    let t38503 = -4.0 / 3.0 * t38461 - 8.0 / 3.0 * t462 * t38464 * t38269 - 4.0 * t462 * t1787 * t38283 + 8.0 * t462 * t8291 * t37273 - 16.0 / 3.0 * t462 * t8327 * t38273 + 8.0 * t462 * t38478 * t38264 + 40.0 / 27.0 * t462 * t38483 * t37306 - 20.0 / 9.0 * t462 * t11690 * t37311 + 16.0 / 9.0 * t38490 + 2.0 * t462 * t1787 * t37283 - 12.0 * t462 * t3134 * t37315 + 8.0 * t462 * t3127 * t37320 + 4.0 / 3.0 * t38501;
    (t38503,)
}
