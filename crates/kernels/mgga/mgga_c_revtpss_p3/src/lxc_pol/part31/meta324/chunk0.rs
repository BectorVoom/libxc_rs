//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1327/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1327<F: Float>(t11670: F, t3089: F, t1087: F, t3090: F, t3278: F, t3182: F, t828: F, t3109: F, t126: F, t3181: F, t1003: F, t3080: F) -> (F, F, F, F, F, F, F) {
    let t11671 = t11670 * t3089;
    let t11672 = t1087 * t11671;
    let t11675 = t3278 * t3090;
    let t11703 = t828 * t3182;
    let t11710 = t828 * t3109;
    let t11725 = t126 * t3181;
    let t11732 = t1003 * t3080;
    (t11671, t11672, t11675, t11703, t11710, t11725, t11732)
}
