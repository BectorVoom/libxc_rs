//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 492/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk492<F: Float>(t2294: F, t259: F, t363: F, t364: F, t358: F, t265: F, t2098: F, t9: F) -> (F, F, F, F, F, F) {
    let t2295 = t259 * t2294;
    let t2298 = t363 * t363;
    let t2300 = 1.0 / t364 / t2298;
    let t2301 = t358 * t2300;
    let t2302 = t2301 * t265;
    let t2304 = 1.0 / t9 / t2098;
    (t2295, t2298, t2300, t2301, t2302, t2304)
}
