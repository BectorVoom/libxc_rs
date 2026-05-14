//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1026/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1026<F: Float>(t1039: F, t5842: F, t586: F, t28: F, t5890: F, t379: F, t6630: F, t9073: F, t446: F, t1969: F, t27068: F, t27073: F, t9049: F, t27064: F, t1369: F, t376: F, t6665: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27091 = t5842 * t1039;
    let t27092 = t586 * t27091;
    let t27094 = t5890 * t28 * t27092;
    let t27096 = t6630 * t379;
    let t27097 = t9073 * t27096;
    let t27098 = t446 * t27097;
    let t27100 = t1969 * t27068;
    let t27101 = t446 * t27100;
    let t27103 = t9049 * t27073;
    let t27104 = t446 * t27103;
    let t27106 = t9073 * t27064;
    let t27107 = t446 * t27106;
    let t27110 = t1369 * t376 * t6665;
    (t27091, t27092, t27094, t27096, t27097, t27098, t27100, t27101, t27103, t27104, t27106, t27107, t27110)
}
