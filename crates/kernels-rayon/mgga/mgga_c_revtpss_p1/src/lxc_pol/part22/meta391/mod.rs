//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1961;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1962;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta391(t1868: f64, t4010: f64, t1353: f64, t13767: f64, t2661: f64, t13756: f64, t13762: f64, t13763: f64, t13765: f64, t1410: f64, t9697: f64, t9705: f64, t9711: f64, t9712: f64, t9716: f64, t9725: f64, t9729: f64, t550: f64, t5658: f64, t543: f64, t3992: f64, t5610: f64, t9775: f64, t1889: f64, t9779: f64, t828: f64, t9954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t13768 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1961(t1868, t4010);
        let (t13769, t13770, t13772, t13773) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1962(t1353, t13768, t13767, t2661, t13756, t13762, t13763, t13765, t1410, t9697, t9705, t9711, t9712, t9716, t9725, t9729);
        let (t13774, t13775, t13776, t13778, t13779, t13781, t13783) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1963(t550, t5658, t543, t3992, t2661, t5610, t9775, t1889, t9779, t828, t9954);
    (t13768, t13769, t13770, t13772, t13773, t13774, t13775, t13776, t13778, t13779, t13781, t13783)
}
