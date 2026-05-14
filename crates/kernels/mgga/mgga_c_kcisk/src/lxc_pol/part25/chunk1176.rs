//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1176/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1176<F: Float>(t34314: F, t34317: F, t34319: F, t34322: F, t34325: F, t34327: F, t34330: F, t34332: F, t34334: F, t34336: F, t34338: F, t34340: F, t34342: F, t2454: F, t733: F, t9709: F) -> (F, F, F) {
    let t34344 = -t34314 / 16.0 + t34317 / 24.0 + t34319 / 96.0 - t34322 / 288.0 - t34325 / 16.0 + t34327 / 24.0 + t34330 / 6.0 + t34332 / 18.0 + t34334 / 128.0 - t34336 / 128.0 + t34338 / 24.0 - t34340 / 24.0 + t34342 / 128.0;
    let t34345 = t733 * t2454;
    let t34346 = t34345 * t9709;
    (t34344, t34345, t34346)
}
