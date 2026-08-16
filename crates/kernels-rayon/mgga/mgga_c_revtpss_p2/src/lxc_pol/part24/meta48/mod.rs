//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk327;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk328;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta48(t421: f64, t1118: f64, t431: f64, t426: f64, t1143: f64, t434: f64, t444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1150 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk327(t421);
        let (t1154, t1159, t1160, t1161, t1163, t1166, t1169) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk328(t1118, t431, t426, t1143, t434);
        let (t1173, t1178, t1179) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk329(t1118, t444);
    (t1150, t1154, t1159, t1160, t1161, t1163, t1166, t1169, t1173, t1178, t1179)
}
