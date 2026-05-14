//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1305/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1305<F: Float>(t118731: F, t1339: F, t9462: F, t110256: F, t113598: F, t113604: F, t113606: F, t118699: F, t118712: F, t118715: F, t118718: F, t118721: F, t118727: F, t118729: F, t9426: F, t9429: F, t9454: F) -> (F, F) {
    let t118733 = t1339 * t118731 * t9462;
    let t118738 = t113598 + t113604 + t113606 - 0.33163888888888888888e-2 * t118712 - 0.73697530864197530861e-2 * t118715 + 0.20833333333333333334e-1 * t118718 * t9454 + 0.69444444444444444447e-2 * t118721 + 0.20833333333333333334e-1 * t118718 * t9429 - 0.16581944444444444444e-2 * t118727 + 0.69444444444444444447e-2 * t118729 + 0.16581944444444444444e-2 * t118733 - 0.23148148148148148149e-2 * t110256 + 0.40208333333333333335e-2 * t9426 * t118699;
    (t118733, t118738)
}
