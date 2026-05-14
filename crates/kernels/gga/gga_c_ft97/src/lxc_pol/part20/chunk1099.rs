//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1099/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1099<F: Float>(t122: F, t6776: F, t1100: F, t96659: F, t2409: F, t27659: F, t27660: F, t10915: F, t229: F, t2418: F, t27528: F, t17836: F, t24371: F, t1113: F, t2378: F, t2395: F) -> (F, F, F, F, F, F, F) {
    let t108950 = t6776 * t122;
    let t108965 = t1100 * t96659;
    let t108969 = t27659 * t27660 * t2409;
    let t108972 = t229 * t10915;
    let t108977 = t27528 * t2418;
    let t108981 = t17836 * t24371;
    let t108983 = t1113 * t2378 * t2395;
    (t108950, t108965, t108969, t108972, t108977, t108981, t108983)
}
