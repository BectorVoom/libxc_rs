//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1329/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1329<F: Float>(t114608: F, t1339: F, t9814: F, t34727: F, t3748: F, t109832: F, t110524: F, t114075: F, t114407: F, t119254: F, t119257: F, t119261: F, t119264: F, t119269: F, t119272: F, t32096: F, t33389: F, t34697: F, t34763: F, t9809: F) -> (F, F, F) {
    let t119277 = t1339 * t114608 * t9814;
    let t119279 = t3748 * t34727;
    let t119283 = -0.46561250000000000002e-2 * t114407 * t33389 - 0.33163888888888888888e-2 * t119254 - 0.11054629629629629629e-2 * t119257 + 0.49745833333333333332e-2 * t119261 + 0.14739506172839506173e-2 * t109832 - 0.34722222222222222223e-2 * t119264 - 0.18518518518518518519e-1 * t110524 * t34763 + 0.23148148148148148149e-2 * t119269 + 0.3684876543209876543e-2 * t119272 + 0.10416666666666666667e-1 * t32096 * t34697 - 0.88437037037037037035e-2 * t119277 + 0.22109259259259259259e-2 * t119279 + 0.20833333333333333334e-1 * t114075 * t9809;
    (t119277, t119279, t119283)
}
