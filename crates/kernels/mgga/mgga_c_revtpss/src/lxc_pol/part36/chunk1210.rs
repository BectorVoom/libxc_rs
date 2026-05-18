//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1210/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1210<F: Float>(t90: F, t29: F, t560: F, t9655: F, t1389: F, t268: F, t10115: F, t555: F, t4146: F, t10308: F, t1466: F, t2246: F, t5812: F) -> (F, F, F, F, F, F, F) {
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = F::new(1.0) / t9655 / t560;
    let t46808 = t1389 * t268;
    let t47567 = t10115 * t555;
    let t47671 = t4146 * t4146;
    let t47672 = F::new(1.0) / t47671;
    let t60224 = t1466 * t10308;
    let t60673 = t5812 * t2246;
    (t45972, t46361, t46808, t47567, t47672, t60224, t60673)
}
