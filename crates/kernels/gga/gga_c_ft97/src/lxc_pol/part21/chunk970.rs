//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 970/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk970<F: Float>(t30232: F, t9236: F, t1369: F, t28: F, t30223: F, t9073: F, t446: F, t1359: F, t4714: F) -> (F, F, F, F, F) {
    let t30233 = t9236 * t30232;
    let t30235 = t1369 * t28 * t30233;
    let t30236 = t9073 * t30223;
    let t30237 = t446 * t30236;
    let t30239 = t1359 * t4714;
    (t30233, t30235, t30236, t30237, t30239)
}
