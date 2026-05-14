//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 196/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk196<F: Float>(t60: F, t116: F, t918: F, t114: F, t126: F, t6: F, t852: F) -> (F, F, F, F, F, F) {
    let t124 = 0.0 < t60;
    let t919 = t116 * t918;
    let t920 = t114 * t919;
    let t923 = t126 * t126;
    let t924 = 1.0 / t923;
    let t925 = t6 * t924;
    let t927 = piecewise3(t124, t852, -t852);
    (t919, t920, t923, t924, t925, t927)
}
