//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2021/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2021<F: Float>(t25374: F, t93341: F, t25378: F, t11050: F, t25399: F, t11007: F, t1955: F, t7056: F, t93320: F, t25387: F, t93330: F, t25410: F, t93189: F) -> (F, F, F, F, F, F, F, F) {
    let t93342 = t93341 * t25374;
    let t93343 = t93342 * t25378;
    let t93346 = t25399 * t11050;
    let t93349 = t1955 * t7056 * t11007;
    let t93364 = t93320 * t25374;
    let t93365 = t93364 * t25378;
    let t93369 = t25387 * t93330;
    let t93371 = t93189 * t25410;
    (t93342, t93343, t93346, t93349, t93364, t93365, t93369, t93371)
}
