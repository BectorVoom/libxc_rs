//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta742 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2613;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta742(t48089: f64, t221: f64, t9817: f64, t13792: f64, t13845: f64, t1882: f64, t9994: f64, t13793: f64, t13999: f64, t1868: f64, t3923: f64, t1353: f64, t13783: f64, t13789: f64, t13790: f64, t13791: f64, t13804: f64, t1398: f64, t21990: f64, t3889: f64, t46592: f64, t46596: f64, t46598: f64, t46600: f64, t46602: f64, t46607: f64, t46613: f64, t46618: f64, t46620: f64, t46622: f64, t46633: f64, t46641: f64, t5671: f64, t9835: f64, t13872: f64, t3978: f64, t9921: f64, t1320: f64, t13632: f64, t13672: f64, t1317: f64, t13680: f64, t3860: f64, t5567: f64, t46960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48090, t48100, t48105, t48113, t48129) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2613(t48089, t221, t9817, t13792, t13845, t1882, t9994, t13793, t13999, t1868, t3923, t1353, t13783, t13789, t13790, t13791, t13804, t1398, t21990, t3889, t46592, t46596, t46598, t46600, t46602, t46607, t46613, t46618, t46620, t46622, t46633, t46641, t5671, t9835);
        let (t48143, t48153, t48155, t48157, t48159, t48160) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2614(t13872, t221, t3978, t9921, t1320, t13632, t13672, t1317, t13680, t3860, t5567, t46960);
    (t48090, t48100, t48105, t48113, t48129, t48143, t48153, t48155, t48157, t48159, t48160)
}
