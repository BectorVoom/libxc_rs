//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 728/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk728<F: Float>(t167: F, t20023: F, t9327: F, t4839: F, t569: F, t925: F, t1053: F, t4668: F, t2185: F, t605: F, t1017: F, t4714: F) -> (F, F, F, F, F) {
    let t20702 = t9327 * t167 * t20023;
    let t20706 = t569 * t4839 * t925;
    let t20709 = t4668 * t1053;
    let t20711 = t2185 * t605 * t20709;
    let t20714 = t1017 * t4714;
    (t20702, t20706, t20709, t20711, t20714)
}
