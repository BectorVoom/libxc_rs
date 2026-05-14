//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 985/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk985<F: Float>(t10179: F, t4147: F, t560: F, t9655: F, t1398: F, t9840: F, t4056: F, t543: F, t1389: F, t268: F, t221: F, t9984: F, t10115: F, t555: F, t4146: F, t1353: F, t4144: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46304 = t10179 * t4147;
    let t46361 = 1.0 / t9655 / t560;
    let t46422 = t9840 * t1398;
    let t46432 = t4056 * t1398;
    let t46433 = t46432 * t543;
    let t46808 = t1389 * t268;
    let t47300 = t221 * t9984;
    let t47567 = t10115 * t555;
    let t47671 = t4146 * t4146;
    let t47672 = 1.0 / t47671;
    let t49560 = t4144 * t1353;
    (t46304, t46361, t46422, t46433, t46808, t47300, t47567, t47672, t49560)
}
