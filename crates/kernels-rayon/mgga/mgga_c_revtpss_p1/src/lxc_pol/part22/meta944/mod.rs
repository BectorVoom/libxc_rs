//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta944 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3180;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta944(t3718: f64, t44546: f64, t5347: f64, t12916: f64, t17785: f64, t5331: f64, t3650: f64, t5390: f64, t12915: f64, t16775: f64, t247: f64, t5384: f64, t12948: f64, t17377: f64, t17361: f64, t3708: f64, t17290: f64, t3678: f64, t3625: f64, t44250: f64, t5401: f64, t127: f64, t5277: f64, t12866: f64, t3630: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58850, t58853, t58863, t58868) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3180(t3718, t44546, t5347, t12916, t17785, t5331, t3650, t5390, t12915, t16775, t247, t5384);
        let (t58878, t58882, t58884, t58889, t58895, t58897) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3181(t12948, t17377, t17361, t3708, t17290, t3678, t3625, t44250, t5401, t127, t5277, t12866, t3630);
    (t58850, t58853, t58863, t58868, t58878, t58882, t58884, t58889, t58895, t58897)
}
