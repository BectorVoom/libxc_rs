//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1342/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1342<F: Float>(t17905: F, t34321: F, t1934: F, t735: F, t17000: F, t17902: F, t34368: F, t117390: F, t117392: F, t117394: F, t117396: F, t117398: F, t117401: F, t117403: F, t117405: F, t117407: F, t117411: F, t117413: F, t117415: F, t117417: F, t117420: F, t117422: F) -> (F, F, F, F) {
    let t117424 = t34321 * t17905;
    let t117426 = t1934 * t735;
    let t117427 = t117426 * t17000;
    let t117429 = t34368 * t17902;
    let t117431 = -t117390 / 32.0 - t117392 / 32.0 + t117394 / 72.0 - t117396 / 128.0 + t117398 / 24.0 + t117401 / 3.0 + t117403 / 48.0 + t117405 / 48.0 + t117407 / 96.0 - t117411 / 9.0 + t117413 / 8.0 + t117415 / 432.0 - t117417 / 12.0 + t117420 / 48.0 + t117422 / 288.0 - t117424 / 288.0 + t117427 / 36.0 - t117429 / 48.0;
    (t117424, t117427, t117429, t117431)
}
