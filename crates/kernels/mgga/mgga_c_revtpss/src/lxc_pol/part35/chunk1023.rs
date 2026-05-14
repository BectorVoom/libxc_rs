//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1023/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1023<F: Float>(t26506: F, t27213: F, t103000: F, t93371: F, t25410: F, t8011: F, t93240: F, t1580: F, t2439: F, t26434: F, t2453: F, t2458: F, t7998: F, t14485: F, t26497: F, t10073: F, t25402: F, t7056: F, t7997: F) -> (F, F, F, F, F, F, F) {
    let t103114 = t27213 * t26506;
    let t103122 = t93371 * t103000;
    let t103130 = t93240 * t25410 * t8011;
    let t103158 = t2439 * t26434 * t1580;
    let t103161 = t2453 * t7998 * t2458;
    let t103220 = t26497 * t14485;
    let t103234 = t10073 * t7056 * t25402 * t7997;
    (t103114, t103122, t103130, t103158, t103161, t103220, t103234)
}
