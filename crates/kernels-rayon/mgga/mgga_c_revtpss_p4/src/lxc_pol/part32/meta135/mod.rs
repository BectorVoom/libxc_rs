//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk699;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk700;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta135(t3478: f64, t3356: f64, t1175: f64, t1179: f64, t1178: f64, t444: f64, t439: f64, t3413: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3479, t3483, t3491, t3495) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk699(t3478, t3356, t1175, t1179, t1178, t444);
        let (t3496, t3503, t3510, t3519, t3520) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk700(t3495, t439, t3356, t3413, t1178);
        let (t3521, t3522, t3523) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk701(t3520, t439, t447);
    (t3479, t3483, t3491, t3495, t3496, t3503, t3510, t3519, t3520, t3521, t3522, t3523)
}
