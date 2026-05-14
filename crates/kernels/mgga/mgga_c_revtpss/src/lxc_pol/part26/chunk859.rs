//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 859/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk859<F: Float>(t1145: F, t12277: F, t141: F, t3362: F, t606: F, t2258: F, t3417: F, t3367: F, t3360: F, t128: F) -> (F, F, F, F, F, F) {
    let t12278 = t1145 * t12277;
    let t12279 = t141 * t12278;
    let t12281 = t3362 * t606;
    let t12282 = t12281 * t2258;
    let t12283 = t3417 * t12282;
    let t12284 = t141 * t12283;
    let t12286 = t3367 * t606;
    let t12287 = t12286 * t2258;
    let t12288 = t1145 * t12287;
    let t12289 = t141 * t12288;
    let t12291 = t3360 * t12282;
    let t12292 = t128 * t12291;
    (t12279, t12282, t12284, t12287, t12289, t12292)
}
