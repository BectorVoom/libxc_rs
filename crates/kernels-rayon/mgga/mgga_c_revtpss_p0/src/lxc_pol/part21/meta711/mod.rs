//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2542;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta711(t3981: f64, t46644: f64, t1412: f64, t3889: f64, t808: f64, t9736: f64, t1369: f64, t9726: f64, t1372: f64, t13999: f64, t9837: f64, t546: f64, t9801: f64, t9738: f64, t124: f64, t3938: f64, t4056: f64, t9816: f64, t9818: f64, t794: f64, t9747: f64, t9750: f64, t2699: f64, t3943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46645, t46649, t46651, t46652, t46660, t46670) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2542(t3981, t46644, t1412, t3889, t808, t9736, t1369, t9726, t1372, t13999, t9837, t546, t9801);
        let (t46671, t46680, t46691, t46692, t46694) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2543(t46670, t9738, t124, t3938, t4056, t9816, t9818, t794, t9747, t9750, t2699, t3943);
    (t46645, t46649, t46651, t46652, t46660, t46670, t46671, t46680, t46691, t46692, t46694)
}
