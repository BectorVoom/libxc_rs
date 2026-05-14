//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 588/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk588<F: Float>(t10: F, t3050: F, t83: F, t1636: F, t433: F, t89: F, t1557: F, t487: F, t1586: F, t355: F, t100: F, t1541: F, t443: F, t444: F) -> (F, F, F, F, F, F, F) {
    let t8189 = t10 * t3050 * t83;
    let t8190 = 14.0 / 81.0 * t8189;
    let t8192 = t89 * t1636 * t433;
    let t8210 = t487 * t1557;
    let t8216 = t355 * t1586;
    let t8217 = t8216 * t100;
    let t8232 = t443 * t444 * t1541;
    (t8189, t8190, t8192, t8210, t8216, t8217, t8232)
}
