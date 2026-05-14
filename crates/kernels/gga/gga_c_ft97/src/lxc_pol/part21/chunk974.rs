//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 974/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk974<F: Float>(t30231: F, t30279: F, t160: F, t24081: F, t4822: F, t24080: F, t165: F, t4668: F, t23400: F, t28: F, t1384: F, t17409: F, t4805: F, t5935: F, t3578: F, t6718: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30280 = t30231 + t30279;
    let t30281 = t30280 * t160;
    let t30284 = t24081 * t4822;
    let t30285 = t24080 * t30284;
    let t30288 = t165 * t4668;
    let t30289 = t23400 * t30288;
    let t30290 = t28 * t30289;
    let t30297 = t17409 * t1384;
    let t30302 = t5935 * t4805;
    let t30304 = t3578 * t6718;
    (t30280, t30281, t30284, t30285, t30288, t30289, t30290, t30297, t30302, t30304)
}
