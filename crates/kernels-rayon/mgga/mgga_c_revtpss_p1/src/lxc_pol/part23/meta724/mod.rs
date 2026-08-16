//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta724 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2488;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta724(t13951: f64, t2713: f64, t3964: f64, t1413: f64, t46835: f64, t48698: f64, t1873: f64, t46651: f64, t13910: f64, t808: f64, t9736: f64, t550: f64, t9794: f64, t14224: f64, t9793: f64, t13800: f64, t46670: f64, t5617: f64, t9732: f64, t136: f64, t216: f64, t9747: f64, t14230: f64, t46802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49008, t49012, t49030, t49057, t49068) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2488(t13951, t2713, t3964, t1413, t46835, t48698, t1873, t46651, t13910, t808, t9736, t550, t9794);
        let (t49071, t49087, t49090, t49093, t49103) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2489(t14224, t49068, t9793, t13800, t46670, t3964, t5617, t9732, t136, t216, t9747, t14230, t46802);
    (t49008, t49012, t49030, t49057, t49071, t49087, t49090, t49093, t49103)
}
