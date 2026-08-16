//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1812;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta582(t6861: f64, t9994: f64, t13789: f64, t13804: f64, t1390: f64, t1410: f64, t1414: f64, t1868: f64, t1882: f64, t1883: f64, t22046: f64, t22079: f64, t23037: f64, t3934: f64, t3936: f64, t48518: f64, t5671: f64, t5673: f64, t6862: f64, t6869: f64, t73929: f64, t73953: f64, t74017: f64, t74024: f64, t828: f64, t85553: f64, t85563: f64, t85638: f64, t85659: f64, t85705: f64, t85735: f64, t85741: f64, t85752: f64, t91826: f64, t9993: f64, t543: f64, t73321: f64, t48152: f64, t73329: f64, t73331: f64, t73341: f64, t39419: f64, t39422: f64, t46292: f64, t46297: f64, t46303: f64, t46963: f64, t46970: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91921, t91922, t91927) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1812(t6861, t9994, t13789, t13804, t1390, t1410, t1414, t1868, t1882, t1883, t22046, t22079, t23037, t3934, t3936, t48518, t5671, t5673, t6862, t6869, t73929, t73953, t74017, t74024, t828, t85553, t85563, t85638, t85659, t85705, t85735, t85741, t85752, t91826, t9993);
        let (t91942, t91952, t91953, t91954, t91955, t91956, t91957) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1813(t543, t91921, t73321, t48152, t73329, t73331, t73341, t39419, t39422, t46292, t46297, t46303, t46963, t46970);
    (t91921, t91922, t91927, t91942, t91952, t91953, t91954, t91955, t91956, t91957)
}
