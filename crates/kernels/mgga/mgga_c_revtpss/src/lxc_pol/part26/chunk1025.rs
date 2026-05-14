//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1025/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1025<F: Float>(t2047: F, t92584: F, t2247: F, t2251: F, t68: F, t25162: F, t26182: F, t6960: F, t92565: F, t92588: F, t95284: F, t95286: F, t95288: F, t95290: F, t95294: F, t95297: F, t95303: F) -> (F,) {
    let t95306 = t2047 * t92584;
    let t95310 = t2247 * t2251 * t68;
    let t95313 = 80.0 / 3.0 * t95284 + 40.0 / 3.0 * t95286 + 32.0 / 3.0 * t95288 + 16.0 / 3.0 * t95290 - 440.0 / 9.0 * t95294 - 160.0 / 3.0 * t95297 + 20.0 * t92565 * t26182 + 10.0 * t92588 * t26182 + 20.0 * t25162 * t95303 + 10.0 * t25162 * t95306 + 10.0 * t95310 * t6960;
    (t95313,)
}
