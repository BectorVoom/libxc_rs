//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1675;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1676;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1677;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta459(t1399: f64, t676: f64, t25894: f64, t25898: f64, t1032: f64, t1419: f64, t1955: f64, t545: f64, t9656: f64, t4075: f64, t7282: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25900, t25904) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1675(t1399, t676, t25894, t25898);
        let (t25920, t25921) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1676(t1032, t1419, t1955);
        let t25924 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1677(t545, t9656);
        let (t25929, t25930) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1678(t4075, t7282, t1955);
    (t25900, t25904, t25920, t25921, t25924, t25929, t25930)
}
