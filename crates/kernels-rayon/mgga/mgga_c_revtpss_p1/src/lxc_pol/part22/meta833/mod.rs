//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta833 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2956;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta833(t13955: f64, t46946: f64, t13775: f64, t808: f64, t9845: f64, t46917: f64, t5701: f64, t14005: f64, t46740: f64, t5697: f64, t1872: f64, t4057: f64, t9816: f64, t9818: f64, t13824: f64, t221: f64, t3978: f64, t46716: f64, t13923: f64, t3930: f64, t14036: f64, t9976: f64, t46694: f64, t5686: f64, t14030: f64, t9744: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48600, t48603, t48614, t48637, t48645, t48655) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2956(t13955, t46946, t13775, t808, t9845, t46917, t5701, t14005, t46740, t5697, t1872, t4057, t9816, t9818);
        let (t48664, t48666, t48668, t48685, t48687) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2957(t13824, t221, t3978, t46716, t13923, t3930, t14036, t9976, t46694, t5686, t14030, t9744);
    (t48600, t48603, t48614, t48637, t48645, t48655, t48664, t48666, t48668, t48685, t48687)
}
