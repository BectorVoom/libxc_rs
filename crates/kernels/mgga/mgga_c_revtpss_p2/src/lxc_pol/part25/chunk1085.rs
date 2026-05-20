//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1085/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1085<F: Float>(t13037: F, t474: F, t11243: F, t479: F, t13036: F, t1248: F, t3601: F, t482: F, t3603: F, t471: F, t11249: F, t1042: F) -> (F, F, F, F, F, F, F) {
    let t13038 = F::new(1.0) / t13037;
    let t13039 = t13038 * t474;
    let t13040 = t479 * t11243;
    let t13041 = t13039 * t13040;
    let t13042 = t13036 * t13041;
    let t13043 = t3601 * t1248;
    let t13044 = t482 * t13043;
    let t13045 = t3603 * t471;
    let t13046 = t11249 * t13045;
    let t13047 = t13044 * t13046;
    let t13048 = t1042 * t13047;
    (t13038, t13040, t13042, t13043, t13044, t13045, t13048)
}
