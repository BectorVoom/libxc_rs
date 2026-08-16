//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1338;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1339;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta329<F: Float>(t2434: F, t371: F, t373: F, t367: F, t1065: F, t675: F, t247: F, t906: F, t1063: F, t1062: F, t3223: F, t1052: F, t3147: F, t1036: F, t3141: F, t3144: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11972, t11986, t11989, t11994, t11997) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1338::<F>(t2434, t371, t373, t367, t1065, t675, t247, t906, t1063, t1062, t3223, t1052, t3147);
        let (t11999, t12013, t12046, t12047, t12050) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1339::<F>(t1036, t11997, t3141, t3144, t1035, t11239, t342, t3145, t334);
    (t11972, t11986, t11989, t11994, t11999, t12013, t12046, t12047, t12050)
}
