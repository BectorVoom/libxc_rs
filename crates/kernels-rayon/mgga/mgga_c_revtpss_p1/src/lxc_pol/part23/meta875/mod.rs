//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta875 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2778;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta875(t2661: f64, t3992: f64, t48533: f64, t6869: f64, t14045: f64, t22096: f64, t21990: f64, t5608: f64, t9934: f64, t1413: f64, t46835: f64, t74483: f64, t22061: f64, t9793: f64, t9794: f64, t22093: f64, t9962: f64, t13845: f64, t73731: f64, t9818: f64, t9835: f64, t13847: f64, t13848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74598, t74602, t74606, t74638) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2778(t2661, t3992, t48533, t6869, t14045, t22096, t21990, t5608, t9934, t1413, t46835, t74483);
        let (t74641, t74656, t74660, t74664) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2779(t22061, t9793, t9794, t22093, t9962, t13845, t73731, t9818, t9835, t13847, t13848, t21990);
    (t74598, t74602, t74606, t74638, t74641, t74656, t74660, t74664)
}
