//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1129/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1129<F: Float>(t25410: F, t93320: F, t25413: F, t7064: F, t93150: F, t25375: F, t93311: F, t122: F, t7048: F, t72: F, t2466: F, t7015: F, t9292: F, t25411: F, t93183: F, t25431: F, t93123: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93321 = t93320 * t25410;
    let t93322 = t93321 * t25413;
    let t93324 = t7064 * t93150;
    let t93326 = t25375 * t93311;
    let t93329 = t7048 * t72 * t122;
    let t93330 = t93329 * t2466;
    let t93331 = t25375 * t93330;
    let t93334 = 0.17073386770573548589e-1 * t9292 * t7015;
    let t93335 = t25411 * t93183;
    let t93337 = t25431 * t93123;
    (t93322, t93324, t93326, t93329, t93330, t93331, t93334, t93335, t93337)
}
