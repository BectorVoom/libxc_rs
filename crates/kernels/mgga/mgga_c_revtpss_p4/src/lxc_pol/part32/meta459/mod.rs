//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1675;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1676;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1677;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta459<F: Float>(t1399: F, t676: F, t25894: F, t25898: F, t1032: F, t1419: F, t1955: F, t545: F, t9656: F, t4075: F, t7282: F) -> (F, F, F, F, F, F, F) {
        let (t25900, t25904) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1675::<F>(t1399, t676, t25894, t25898);
        let (t25920, t25921) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1676::<F>(t1032, t1419, t1955);
        let t25924 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1677::<F>(t545, t9656);
        let (t25929, t25930) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1678::<F>(t4075, t7282, t1955);
    (t25900, t25904, t25920, t25921, t25924, t25929, t25930)
}
