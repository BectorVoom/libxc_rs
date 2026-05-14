//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 344/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk344<F: Float>(t1211: F, t1214: F, t139: F, t221: F, t462: F, t461: F, t1010: F, t56: F) -> (F, F, F) {
    let t1215 = t1211 * t1214;
    let t1219 = t221 * t139 * t462;
    let t1221 = t461 * t1219 / 288.0;
    let t1222 = t56 * t1010;
    (t1215, t1221, t1222)
}
