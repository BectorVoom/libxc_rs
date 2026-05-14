//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1096/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1096<F: Float>(t2337: F, t2881: F, t3424: F, t3685: F, t40443: F, t40444: F, t40446: F, t40448: F, t40463: F, t40467: F, t40469: F, t40471: F, t40475: F, t40479: F, t40483: F, t40490: F, t40495: F, t40502: F) -> (F,) {
    let t40735 = t2337 * t3685 + 2.0 * t2881 * t3424 + t40443 + t40444 - t40446 - t40448 - t40463 - t40467 + t40469 + t40471 - t40475 - t40479 + t40483 - t40490 - t40495 + t40502;
    (t40735,)
}
