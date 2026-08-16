//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1487;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta491(t22021: f64, t9793: f64, t9794: f64, t6876: f64, t9909: f64, t22026: f64, t46929: f64, t808: f64, t22259: f64, t9976: f64, t22125: f64, t2713: f64, t3964: f64, t1868: f64, t1883: f64, t46825: f64, t22126: f64, t2689: f64, t22130: f64, t22056: f64, t9765: f64, t9845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74341, t74358, t74362, t74429, t74437) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1487(t22021, t9793, t9794, t6876, t9909, t22026, t46929, t808, t22259, t9976, t22125, t2713, t3964);
        let (t74483, t74485, t74491, t74493, t74511, t74522) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1488(t1868, t1883, t46825, t9793, t22126, t2689, t22130, t22056, t9765, t22021, t808, t9845);
    (t74341, t74358, t74362, t74429, t74437, t74483, t74485, t74491, t74493, t74511, t74522)
}
