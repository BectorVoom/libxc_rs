//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 965/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk965<F: Float>(t37716: F, t546: F, t20339: F, t565: F, t19853: F, t146: F, t2078: F, t2206: F, t10772: F, t10810: F, t2141: F, t269: F, t572: F) -> (F, F, F, F, F, F, F) {
    let t37717 = t546 * t37716;
    let t37718 = t37717 * t20339;
    let t37720 = t565 * t37716;
    let t37721 = t37720 * t19853;
    let t37736 = t146 * t2206 * t2078;
    let t37749 = t10772 * t10810 * t2141;
    let t37754 = t572 * t269;
    (t37717, t37718, t37720, t37721, t37736, t37749, t37754)
}
