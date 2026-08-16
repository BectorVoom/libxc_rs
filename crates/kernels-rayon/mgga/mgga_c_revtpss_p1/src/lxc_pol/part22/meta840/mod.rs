//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta840 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2970;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta840(t13981: f64, t9962: f64, t13951: f64, t2713: f64, t3964: f64, t1413: f64, t46835: f64, t48698: f64, t13845: f64, t13847: f64, t13848: f64, t4004: f64, t1872: f64, t9818: f64, t1873: f64, t46651: f64, t1399: f64, t5689: f64, t9816: f64, t3924: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49005, t49008, t49012, t49016) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2970(t13981, t9962, t13951, t2713, t3964, t1413, t46835, t48698, t13845, t13847, t13848, t4004);
        let (t49024, t49030, t49049, t49053) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2971(t13845, t1872, t4004, t9818, t1873, t46651, t1399, t5689, t9816, t13847, t13848, t3924);
    (t49005, t49008, t49012, t49016, t49024, t49030, t49049, t49053)
}
