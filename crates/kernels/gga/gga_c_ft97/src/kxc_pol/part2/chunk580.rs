//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 580/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk580<F: Float>(t1586: F, t355: F, t100: F, t1882: F, t1917: F, t1878: F, t1541: F, t443: F, t444: F) -> (F, F, F, F, F) {
    let t8216 = t355 * t1586;
    let t8217 = t8216 * t100;
    let t8227 = t1882 * t1917;
    let t8229 = t1882 * t1878;
    let t8232 = t443 * t444 * t1541;
    (t8216, t8217, t8227, t8229, t8232)
}
