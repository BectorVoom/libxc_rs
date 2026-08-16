//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1812;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta582<F: Float>(t6861: F, t9994: F, t13789: F, t13804: F, t1390: F, t1410: F, t1414: F, t1868: F, t1882: F, t1883: F, t22046: F, t22079: F, t23037: F, t3934: F, t3936: F, t48518: F, t5671: F, t5673: F, t6862: F, t6869: F, t73929: F, t73953: F, t74017: F, t74024: F, t828: F, t85553: F, t85563: F, t85638: F, t85659: F, t85705: F, t85735: F, t85741: F, t85752: F, t91826: F, t9993: F, t543: F, t73321: F, t48152: F, t73329: F, t73331: F, t73341: F, t39419: F, t39422: F, t46292: F, t46297: F, t46303: F, t46963: F, t46970: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91921, t91922, t91927) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1812::<F>(t6861, t9994, t13789, t13804, t1390, t1410, t1414, t1868, t1882, t1883, t22046, t22079, t23037, t3934, t3936, t48518, t5671, t5673, t6862, t6869, t73929, t73953, t74017, t74024, t828, t85553, t85563, t85638, t85659, t85705, t85735, t85741, t85752, t91826, t9993);
        let (t91942, t91952, t91953, t91954, t91955, t91956, t91957) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1813::<F>(t543, t91921, t73321, t48152, t73329, t73331, t73341, t39419, t39422, t46292, t46297, t46303, t46963, t46970);
    (t91921, t91922, t91927, t91942, t91952, t91953, t91954, t91955, t91956, t91957)
}
