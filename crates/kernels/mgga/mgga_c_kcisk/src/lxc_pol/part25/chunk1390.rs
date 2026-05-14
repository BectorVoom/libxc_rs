//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1390/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1390<F: Float>(t110907: F, t110912: F, t111201: F, t111203: F, t111524: F, t113294: F, t113307: F, t18936: F, t2356: F, t2359: F, t2776: F, t32873: F, t32882: F, t5556: F, t566: F, t9904: F) -> (F,) {
    let t118587 = t111524 + t2356 * t32882 / 8.0 + t110907 - t110912 + t113294 - t2776 * t2359 * t5556 / 16.0 - t2776 * t566 * t18936 / 16.0 + t9904 * t32873 / 8.0 - t111201 + t113307 + t111203;
    (t118587,)
}
