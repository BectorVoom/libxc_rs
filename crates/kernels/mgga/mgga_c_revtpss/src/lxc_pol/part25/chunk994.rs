//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 994/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk994<F: Float>(t12845: F, t12929: F, t13005: F, t13105: F, t489: F, t1269: F, t3601: F, t3769: F, t1248: F, t1287: F, t3727: F, t3584: F, t3759: F, t11239: F, t1243: F, t460: F) -> (F, F, F, F, F, F, F) {
    let t13107 = t12845 + t12929 + t13005 + t13105;
    let t13108 = t489 * t13107;
    let t13111 = t1269 * t3601;
    let t13112 = t13111 * t3769;
    let t13118 = t3727 * t1248 * t1287;
    let t13121 = t3759 * t3584;
    let t13126 = t11239 * t1243;
    let t13127 = t460 * t13126;
    (t13107, t13108, t13111, t13112, t13118, t13121, t13127)
}
