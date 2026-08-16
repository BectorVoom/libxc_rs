//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2364;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2365;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta548(t17694: f64, t17695: f64, t13396: f64, t5302: f64, t1042: f64, t3588: f64, t3603: f64, t5332: f64, t3720: f64, t15904: f64, t3623: f64, t13148: f64, t11249: f64, t1794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17696, t17699, t17700, t17703, t17704, t17705, t17708) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2364(t17694, t17695, t13396, t5302, t1042, t3588, t3603, t5332, t3720, t15904, t3623);
        let t17709 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2365(t13148, t17708);
        let t17710 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2366(t11249, t1794);
    (t17696, t17699, t17700, t17703, t17704, t17705, t17708, t17709, t17710)
}
