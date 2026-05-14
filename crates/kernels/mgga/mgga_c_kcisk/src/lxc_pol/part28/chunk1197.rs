//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1197/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1197<F: Float>(t34368: F, t7431: F, t1954: F, t6719: F, t34346: F, t34348: F, t34350: F, t34352: F, t34354: F, t34356: F, t34358: F, t34360: F, t34362: F, t34364: F, t34366: F, t34344: F) -> (F, F, F) {
    let t34369 = t34368 * t7431;
    let t34371 = t6719 * t1954;
    let t34373 = -t34346 / 9.0 + t34348 / 96.0 - t34350 / 16.0 - t34352 / 24.0 + t34354 / 128.0 - t34356 / 96.0 + t34358 / 16.0 - t34360 / 6.0 + t34362 / 8.0 - t34364 / 72.0 - t34366 / 24.0 - t34369 / 64.0 - t34371 / 96.0;
    let t34374 = t34344 + t34373;
    (t34369, t34371, t34374)
}
