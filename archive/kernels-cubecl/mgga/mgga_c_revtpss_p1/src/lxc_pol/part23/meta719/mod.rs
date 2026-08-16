//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta719 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2478;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta719<F: Float>(t48486: F, t13985: F, t46740: F, t13878: F, t9765: F, t14055: F, t9775: F, t1885: F, t46722: F, t14047: F, t14051: F, t1412: F, t5658: F, t1389: F, t1882: F, t46856: F, t543: F, t685: F, t72: F, t13955: F, t46946: F, t13775: F, t808: F, t9845: F, t46917: F, t5701: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48487, t48489, t48509, t48516, t48518, t48529, t48532, t48533) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2478::<F>(t48486, t13985, t46740, t13878, t9765, t14055, t9775, t1885, t46722, t14047, t14051, t1412, t5658);
        let (t48563, t48600, t48604, t48614) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2479::<F>(t1389, t1882, t46856, t543, t685, t72, t13955, t46946, t13775, t808, t9845, t46917, t5701);
    (t48487, t48489, t48509, t48516, t48518, t48529, t48532, t48533, t48563, t48600, t48604, t48614)
}
