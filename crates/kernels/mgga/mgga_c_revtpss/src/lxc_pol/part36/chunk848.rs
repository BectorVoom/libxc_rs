//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 848/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk848<F: Float>(t6132: F, t698: F, t6135: F, t6138: F, t300: F, t6184: F, t6104: F, t914: F, t3336: F, t6396: F, t964: F, t6152: F, t945: F, t2970: F, t6173: F, t3014: F, t6205: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19002 = t698 * t6132;
    let t19004 = t698 * t6135;
    let t19009 = t698 * t6138;
    let t19049 = t300 * t6184;
    let t19056 = t6104 * t914;
    let t19153 = t6396 * t3336;
    let t19156 = t6184 * t964;
    let t19173 = t6152 * t945;
    let t19275 = t6173 * t2970;
    let t19303 = t6205 * t3014;
    (t19002, t19004, t19009, t19049, t19056, t19153, t19156, t19173, t19275, t19303)
}
