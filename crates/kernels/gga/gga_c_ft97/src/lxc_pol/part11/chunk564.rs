//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 564/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk564<F: Float>(t100: F, t8216: F, t103: F, t1588: F, t379: F, t1922: F, t447: F, t1882: F, t1917: F, t1878: F, t1541: F, t443: F, t444: F) -> (F, F, F, F, F, F, F) {
    let t8217 = t8216 * t100;
    let t8219 = t103 * t1588 * t379;
    let t8220 = t8217 * t8219;
    let t8224 = t447 * t1922 * t379;
    let t8227 = t1882 * t1917;
    let t8229 = t1882 * t1878;
    let t8232 = t443 * t444 * t1541;
    (t8217, t8219, t8220, t8224, t8227, t8229, t8232)
}
