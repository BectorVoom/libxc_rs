//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 719/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk719<F: Float>(t11337: F, t240: F, t3252: F, t276: F, t285: F, t273: F, t2922: F, t913: F, t275: F, t290: F, t2925: F, t2966: F, t307: F, t302: F, t11132: F, t944: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11338 = 0.36514074074074074075e0 * t11337;
    let t11341 = t240 * t3252;
    let t11354 = 1.0 / t276 / t285 / 4.0;
    let t11358 = 1.0/pow_3_2(t273);
    let t11384 = 1.0 / t2922 / t913;
    let t11385 = t275 * t11384;
    let t11387 = 1.0 / t2925 / t290;
    let t11408 = 1.0 / t2966 / t307;
    let t11409 = t302 * t11408;
    let t11422 = 0.16068111111111111111e1 * t11132;
    let t11423 = 0.46308888888888888888e0 * t11337;
    let t11449 = 1.0 / t2966 / t944;
    (t11338, t11341, t11354, t11358, t11385, t11387, t11409, t11422, t11423, t11449)
}
