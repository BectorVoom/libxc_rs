//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 914/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk914<F: Float>(t1559: F, t23909: F, t9049: F, t446: F, t1882: F, t5918: F, t23884: F, t526: F, t27: F, t89: F, t375: F, t5925: F, t1374: F, t1636: F, t1984: F, t5842: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23910 = t23909 * t1559;
    let t23911 = t9049 * t23910;
    let t23912 = t446 * t23911;
    let t23914 = t1882 * t5918;
    let t23916 = t526 * t23884;
    let t23918 = t89 * t27 * t23916;
    let t23920 = t89 * t375 * t5925;
    let t23923 = t89 * t1636 * t1374;
    let t23924 = 4.0 / 9.0 * t23923;
    let t23925 = t1984 * t5842;
    (t23911, t23912, t23914, t23916, t23918, t23920, t23923, t23924, t23925)
}
