//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1769;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1770;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1771;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1772;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta373(t3361: f64, t57: f64, t10356: f64, t3417: f64, t141: f64, t3362: f64, t1145: f64, t10326: f64, t1121: f64, t606: f64, t2258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12267, t12268) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1769(t3361, t57);
        let t12269 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1770(t10356, t12268);
        let (t12270, t12271, t12273) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1771(t12269, t3417, t141, t10356, t3362);
        let (t12274, t12275, t12277) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1772(t1145, t12273, t141, t10326, t1121);
        let (t12278, t12279, t12282) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1773(t1145, t12277, t141, t3362, t606, t2258);
    (t12267, t12268, t12269, t12270, t12271, t12273, t12274, t12275, t12277, t12278, t12279, t12282)
}
