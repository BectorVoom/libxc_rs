//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk890;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk891;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk892;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta138(t1401: f64, t3930: f64, t1386: f64, t241: f64, t820: f64, t1412: f64, t72: f64, t245: f64, t1353: f64, t543: f64, t159: f64, t550: f64, t216: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3931, t3934) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk890(t1401, t3930, t1386, t241, t820);
        let (t3935, t3936) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk891(t1412, t72, t245);
        let t3938 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk892(t1353, t543);
        let (t3943, t3944) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk893(t159, t550, t216);
    (t3931, t3934, t3935, t3936, t3938, t3943, t3944)
}
