//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 858/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk858<F: Float>(t1349: F, t23930: F, t2252: F, t342: F, t7302: F, t32679: F, t630: F, t24094: F, t7309: F, t32691: F, t376: F, t1637: F, t7341: F, t32686: F, t5769: F, t24116: F) -> (F, F, F, F, F, F, F, F) {
    let t138625 = t1349 * t23930;
    let t138629 = t342 * t2252 * t7302 / 18.0;
    let t138635 = t342 * t630 * t32679;
    let t138652 = t7309 * t24094;
    let t138655 = t1349 * t376 * t32691;
    let t138662 = 2.0 / 27.0 * t1349 * t1637 * t7341;
    let t138677 = t32686 * t5769;
    let t138681 = 2.0 / 27.0 * t7309 * t24116;
    (t138625, t138629, t138635, t138652, t138655, t138662, t138677, t138681)
}
