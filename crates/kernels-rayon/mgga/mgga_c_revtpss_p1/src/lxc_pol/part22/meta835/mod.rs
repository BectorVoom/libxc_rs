//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta835 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2960;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta835(t2661: f64, t3992: f64, t4057: f64, t5608: f64, t4004: f64, t5651: f64, t9934: f64, t47198: f64, t5665: f64, t5629: f64, t9779: f64, t5661: f64, t9909: f64, t47247: f64, t828: f64, t13967: f64, t9962: f64, t13941: f64, t46740: f64, t221: f64, t47273: f64, t13785: f64, t9816: f64, t13770: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48786, t48790, t48792, t48794, t48796) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2960(t2661, t3992, t4057, t5608, t4004, t5651, t9934, t47198, t5665, t5629, t9779, t5661, t9909);
        let (t48798, t48811, t48813, t48823, t48825, t48827) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2961(t47247, t828, t13967, t9962, t13941, t46740, t221, t47273, t13785, t9816, t13770, t9775);
    (t48786, t48790, t48792, t48794, t48796, t48798, t48811, t48813, t48823, t48825, t48827)
}
