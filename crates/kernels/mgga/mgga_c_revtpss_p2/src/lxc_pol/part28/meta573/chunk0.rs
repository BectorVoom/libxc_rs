//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2036/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2036<F: Float>(t25569: F, t3111: F, t11722: F, t7132: F, t11727: F, t12002: F, t1971: F, t351: F, t1052: F, t3089: F, t1087: F, t11744: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t93579 = t25569 * t3111;
    let t93583 = t7132 * t11722;
    let t93585 = t7132 * t11727;
    let t93592 = t351 * t1971 * t12002;
    let t93595 = sigma0 * t1052;
    let t93596 = t93595 * t3089;
    let t93597 = t1087 * t93596;
    let t93602 = t7132 * t11744;
    (t93579, t93583, t93585, t93592, t93595, t93596, t93597, t93602)
}
