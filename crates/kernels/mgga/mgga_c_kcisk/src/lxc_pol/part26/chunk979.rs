//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 979/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk979<F: Float>(t6157: F, t6196: F, t25: F, t8049: F, t1309: F, t20084: F, t20236: F, t20240: F, t20613: F, t2170: F, t26471: F, t26478: F, t3966: F, t3970: F, t8050: F, t8056: F) -> (F,) {
    let t26485 = t6157 * t6196;
    let t26489 = t25 * t8049;
    let t26490 = t1309 * t26489;
    let t26492 = -0.17990788716177317213e-1 * t26471 - 0.5397236614853195164e-1 * t3966 * t8056 - t20236 + t20240 - 0.5397236614853195164e-1 * t1309 * t26478 + 0.14392630972941853771e0 * t3970 * t8056 + 0.28785261945883707541e0 * t20084 * t2170 - 0.35981577432354634427e-1 * t26485 - 0.28785261945883707541e0 * t3970 * t8050 + 0.35981577432354634427e-1 * t26490 + t20613;
    (t26492,)
}
