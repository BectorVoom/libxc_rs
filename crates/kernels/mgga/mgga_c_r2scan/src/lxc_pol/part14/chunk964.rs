//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 964/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk964<F: Float>(t1575: F, t2096: F, t571: F, t10710: F, t20665: F, t3342: F, t572: F, t546: F, t20339: F, t565: F, t19853: F, t146: F, t2078: F, t2206: F, t10772: F, t10810: F, t2141: F) -> (F, F, F, F, F, F, F, F) {
    let t37712 = t571 * t1575 * t2096;
    let t37714 = t37712 * t10710 * t20665;
    let t37716 = t572 * t3342;
    let t37717 = t546 * t37716;
    let t37718 = t37717 * t20339;
    let t37720 = t565 * t37716;
    let t37721 = t37720 * t19853;
    let t37736 = t146 * t2206 * t2078;
    let t37749 = t10772 * t10810 * t2141;
    (t37714, t37716, t37717, t37718, t37720, t37721, t37736, t37749)
}
