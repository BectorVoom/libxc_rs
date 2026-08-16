//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2480;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta720(t48614: f64, t14005: f64, t46740: f64, t46917: f64, t5697: f64, t14036: f64, t9976: f64, t46694: f64, t5686: f64, t13769: f64, t808: f64, t9736: f64, t13952: f64, t2689: f64, t13784: f64, t543: f64, t46825: f64, t9793: f64, t1353: f64, t1883: f64, t1408: f64, t241: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48615, t48638, t48645, t48669, t48686, t48690) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2480(t48614, t14005, t46740, t46917, t5697, t14036, t9976, t46694, t5686, t13769, t808, t9736);
        let (t48691, t48692, t48694, t48696, t48698, t48700, t48712) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2481(t48690, t13952, t2689, t13784, t543, t46825, t9793, t1353, t1883, t1408, t241, t820);
    (t48615, t48638, t48645, t48669, t48686, t48691, t48692, t48694, t48696, t48698, t48700, t48712)
}
