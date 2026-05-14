//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1047/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1047<F: Float>(t1675: F, t25538: F, t27448: F, t27460: F, t27462: F, t27471: F, t27489: F, t29779: F, t375: F, t6285: F, t6289: F, t6293: F, t6323: F, t6327: F, t7111: F, t7132: F) -> (F,) {
    let t29782 = 0.57165357490759649296e-3 * t27448 + t27460 / 432.0 + 0.57165357490759649296e-3 * t27462 - 0.57165357490759649296e-3 * t27471 - t7111 * t6285 / 144.0 + t7111 * t6289 / 288.0 + t7111 * t6293 / 216.0 + 0.28582678745379824648e-3 * t7132 * t6323 + 0.47637797908966374413e-3 * t7132 * t6327 + 0.57165357490759649296e-3 * t27489 * t1675 - t25538 + 0.42874018118069736972e-3 * t29779 * t375;
    (t29782,)
}
