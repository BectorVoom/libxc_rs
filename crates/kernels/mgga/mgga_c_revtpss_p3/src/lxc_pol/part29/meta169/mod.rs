//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk817;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta169<F: Float>(t1122: F, t3634: F, t247: F, t1261: F, t1264: F, t3372: F, t3368: F, t1230: F, t1260: F, t225: F, t3552: F, t480: F, t371: F, t482: F, t676: F, t481: F, t1231: F, t1256: F, t1247: F, t1266: F, t3591: F, t3600: F, t3606: F, t3610: F, t3613: F, t3620: F, t3625: F, t3631: F, t484: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3636, t3637, t3640, t3644, t3647) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk817::<F>(t1122, t3634, t247, t1261, t1264, t3372, t3368, t1230, t1260);
        let (t3650, t3651, t3655, t3657, t3658, t3660) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk818::<F>(t225, t3552, t480, t371, t482, t676, t481, t1231, t1256, t1247, t1261, t1266, t3591, t3600, t3606, t3610, t3613, t3620, t3625, t3631, t3637, t3640, t3644, t3647, t484);
    (t3636, t3637, t3640, t3644, t3647, t3650, t3651, t3655, t3657, t3658, t3660)
}
