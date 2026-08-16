//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta709 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2538;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta709(t2782: f64, t4086: f64, t46394: f64, t543: f64, t3829: f64, t4010: f64, t808: f64, t9736: f64, t1408: f64, t820: f64, t9948: f64, t1416: f64, t9775: f64, t9931: f64, t3989: f64, t9757: f64, t9761: f64, t9765: f64, t1353: f64, t13767: f64, t2661: f64, t3889: f64, t240: f64, t9991: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46587, t46592, t46595, t46596) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2538(t2782, t4086, t46394, t543, t3829, t4010, t808, t9736, t1408, t820, t9948, t1416);
        let (t46598, t46600, t46602, t46607, t46609) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2539(t9775, t9931, t3989, t9757, t9761, t9765, t1353, t13767, t2661, t3889, t4010, t240, t9991);
    (t46587, t46592, t46595, t46596, t46598, t46600, t46602, t46607, t46609)
}
