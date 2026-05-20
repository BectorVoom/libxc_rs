//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2323;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2324;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2325;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta531<F: Float>(t17331: F, t225: F, t480: F, t1256: F, t5258: F, t5262: F, t1804: F, t3655: F, t1786: F, t1260: F, t12987: F, t1774: F, t3568: F, t247: F, t3719: F, t15687: F, t3623: F, t3782: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17332, t17333, t17337, t17339, t17340, t17342, t17344) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2323::<F>(t17331, t225, t480, t1256, t5258, t5262, t1804, t3655, t1786, t1260, t12987);
        let t17345 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2324::<F>(t1774, t3568);
        let (t17347, t17350) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2325::<F>(t17345, t247, t3719, t15687, t3623);
        let t17351 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2326::<F>(t17350, t3782);
    (t17332, t17333, t17337, t17339, t17340, t17342, t17344, t17345, t17347, t17350, t17351)
}
