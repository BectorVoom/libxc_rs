//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 471/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk471<F: Float>(t1526: F, t1529: F, t7705: F, t1533: F, t342: F, t630: F, t23: F, t7241: F, t174: F, t358: F, t1642: F, t369: F, t1586: F, t378: F) -> (F, F, F, F, F, F) {
    let t7707 = t1526 * t7705 * t1529;
    let t7710 = t342 * t630 * t1533;
    let t7750 = t23 * t7241;
    let t7760 = 1.0 / t174 / t358;
    let t7793 = t1642 * t369;
    let t7824 = t378 * t1586;
    (t7707, t7710, t7750, t7760, t7793, t7824)
}
