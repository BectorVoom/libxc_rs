//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1107/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1107<F: Float>(t90: F, t29: F, t10414: F, t116: F, t10179: F, t4147: F, t560: F, t9655: F, t1398: F, t9840: F, t4056: F, t543: F) -> (F, F, F, F, F, F) {
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46126 = t10414 * t116;
    let t46304 = t10179 * t4147;
    let t46361 = F::new(1.0) / t9655 / t560;
    let t46422 = t9840 * t1398;
    let t46432 = t4056 * t1398;
    let t46433 = t46432 * t543;
    (t45972, t46126, t46304, t46361, t46422, t46433)
}
