//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta794 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2889;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2890;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta794(t9761: f64, t9765: f64, t240: f64, t9991: f64, t3995: f64, t40488: f64, t549: f64, t72: f64, t4014: f64, t9779: f64, t1408: f64, t2237: f64, t2482: f64, t3981: f64, t1412: f64, t3889: f64, t808: f64, t9736: f64, t1369: f64, t9726: f64, t1372: f64, t546: f64, t9801: f64, t9738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46602, t46609, t46620, t46627, t46633, t46644) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2889(t9761, t9765, t240, t9991, t3995, t40488, t549, t72, t4014, t9779, t1408, t2237, t2482);
        let (t46645, t46649, t46651, t46652, t46670, t46671) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2890(t3981, t46644, t1412, t3889, t808, t9736, t1369, t9726, t1372, t546, t9801, t9738);
    (t46602, t46609, t46620, t46627, t46633, t46644, t46645, t46649, t46651, t46652, t46670, t46671)
}
