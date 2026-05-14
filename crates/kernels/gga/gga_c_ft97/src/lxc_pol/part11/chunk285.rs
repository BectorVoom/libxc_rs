//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 285/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk285<F: Float>(t550: F, t6: F, t133: F, t342: F, t344: F, t630: F, t11: F, t341: F, t339: F) -> (F, F, F, F) {
    let t1354 = t550 * t6;
    let t1355 = t133 * t1354;
    let t1524 = t342 * t630 * t344 / 12.0;
    let t1525 = t341 * t11;
    let t1526 = t339 * t1525;
    (t1354, t1355, t1524, t1526)
}
