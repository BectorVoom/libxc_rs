//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta719 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2478;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta719(t48486: f64, t13985: f64, t46740: f64, t13878: f64, t9765: f64, t14055: f64, t9775: f64, t1885: f64, t46722: f64, t14047: f64, t14051: f64, t1412: f64, t5658: f64, t1389: f64, t1882: f64, t46856: f64, t543: f64, t685: f64, t72: f64, t13955: f64, t46946: f64, t13775: f64, t808: f64, t9845: f64, t46917: f64, t5701: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48487, t48489, t48509, t48516, t48518, t48529, t48532, t48533) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2478(t48486, t13985, t46740, t13878, t9765, t14055, t9775, t1885, t46722, t14047, t14051, t1412, t5658);
        let (t48563, t48600, t48604, t48614) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2479(t1389, t1882, t46856, t543, t685, t72, t13955, t46946, t13775, t808, t9845, t46917, t5701);
    (t48487, t48489, t48509, t48516, t48518, t48529, t48532, t48533, t48563, t48600, t48604, t48614)
}
