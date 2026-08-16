//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1642;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1643;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta333(t1042: f64, t11285: f64, t2866: f64, t914: f64, t936: f64, t2869: f64, t2919: f64, t2923: f64, t910: f64, t2927: f64, t287: f64, t2922: f64, t275: f64, t2875: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11286, t11289, t11291, t11293, t11294) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1642(t1042, t11285, t2866, t914, t936, t2869, t2919, t2923, t910);
        let (t11296, t11298, t11299) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1643(t11294, t2927, t287, t2922, t275);
        let t11300 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1644(t2875, t934);
    (t11286, t11289, t11291, t11293, t11294, t11296, t11298, t11299, t11300)
}
