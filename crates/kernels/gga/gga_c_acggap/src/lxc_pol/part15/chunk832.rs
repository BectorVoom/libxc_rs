//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 832/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk832<F: Float>(t2100: F, t31289: F, t31228: F, t7556: F, t30984: F, t7495: F, t151: F, t394: F, t592: F, t7510: F, t30402: F, t407: F, t7325: F, t30409: F, t30418: F, t30546: F, t7428: F) -> (F, F, F, F, F, F, F) {
    let t31290 = t31289 * t2100;
    let t31295 = t31228 * t7556;
    let t31297 = t30984 * t7495;
    let t31309 = t151 * t394 * t592 * t7510;
    let t31312 = t31309 * t30402 * t7325 * t407;
    let t31316 = t31309 * t30418 * t30409 * t407;
    let t31318 = t30546 * t7428;
    (t31290, t31295, t31297, t31309, t31312, t31316, t31318)
}
