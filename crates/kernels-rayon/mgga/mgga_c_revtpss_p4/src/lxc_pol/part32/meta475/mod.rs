//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1707;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1708;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta475(t2061: f64, t25402: f64, t7056: f64, t10073: f64, t26544: f64, t7064: f64, t7384: f64, t887: f64, t689: f64, t7399: f64, t786: f64, t789: f64, t2062: f64, t2453: f64, t2458: f64, t2411: f64, t7427: f64, t11064: f64, t2070: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26554, t26555, t26557, t26558, t26560, t26561, t26563, t26564) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1707(t2061, t25402, t7056, t10073, t26544, t7064, t7384, t887, t689, t7399, t786, t789);
        let (t26576, t26578, t26585) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1708(t2062, t2453, t2458, t2411, t7427);
        let t26590 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1709(t11064, t2070);
    (t26554, t26555, t26557, t26558, t26560, t26561, t26563, t26564, t26576, t26578, t26585, t26590)
}
