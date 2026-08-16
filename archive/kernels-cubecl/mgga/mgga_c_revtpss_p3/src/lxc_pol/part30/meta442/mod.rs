//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1694;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta442<F: Float>(t1774: F, t3568: F, t247: F, t3719: F, t15687: F, t3623: F, t3782: F, t1263: F, t1794: F, t372: F, t12712: F, t3629: F, t17301: F, t17304: F, t17308: F, t17311: F, t17333: F, t17337: F, t17339: F, t17340: F, t17342: F, t17344: F, t3674: F, t484: F) -> (F, F, F, F, F, F) {
        let (t17345, t17347, t17350, t17351, t17353, t17354) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1694::<F>(t1774, t3568, t247, t3719, t15687, t3623, t3782, t1263, t1794, t372, t12712, t3629);
        let (t17355, t17358) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1695::<F>(t17353, t17354, t17301, t17304, t17308, t17311, t17333, t17337, t17339, t17340, t17342, t17344, t17347, t17351, t3674, t484);
    (t17345, t17347, t17350, t17353, t17355, t17358)
}
