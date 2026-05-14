//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 935/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk935<F: Float>(t120361: F, t994: F, t11921: F, t247: F, t31920: F, t31921: F, t31964: F, t370: F, t8499: F, t32009: F, t93982: F, t120334: F, t7150: F, t11922: F, t31975: F, t31977: F) -> (F, F, F, F, F, F) {
    let t120532 = t994 * t120361;
    let t120538 = t31920 * t247 * t11921 * t31921;
    let t120555 = t8499 * t31964 * t370;
    let t120558 = t32009 * t93982;
    let t120569 = t7150 * t120334;
    let t120578 = t31975 * t11922 * t31977;
    (t120532, t120538, t120555, t120558, t120569, t120578)
}
