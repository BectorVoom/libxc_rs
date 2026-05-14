//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 846/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk846<F: Float>(t31228: F, t7556: F, t30984: F, t7495: F, t30543: F, t7871: F, t2009: F, t968: F, t151: F, t394: F, t592: F, t7510: F, t30402: F, t407: F, t7325: F, t30409: F, t30418: F) -> (F, F, F, F, F, F, F) {
    let t31295 = t31228 * t7556;
    let t31296 = 0.94344276868812456204e-3 * t31295;
    let t31297 = t30984 * t7495;
    let t31299 = t30543 * t7871;
    let t31305 = t2009 * t968;
    let t31309 = t151 * t394 * t592 * t7510;
    let t31312 = t31309 * t30402 * t7325 * t407;
    let t31316 = t31309 * t30418 * t30409 * t407;
    (t31296, t31297, t31299, t31305, t31309, t31312, t31316)
}
