//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1923;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta428<F: Float>(t1868: F, t4010: F, t1353: F, t13767: F, t2661: F, t13756: F, t13762: F, t13763: F, t13765: F, t1410: F, t9697: F, t9705: F, t9711: F, t9712: F, t9716: F, t9725: F, t9729: F, t550: F, t5658: F, t543: F, t3992: F, t5610: F, t9775: F, t1889: F, t9779: F, t828: F, t9954: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13768, t13769, t13770, t13773) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1923::<F>(t1868, t4010, t1353, t13767, t2661, t13756, t13762, t13763, t13765, t1410, t9697, t9705, t9711, t9712, t9716, t9725, t9729);
        let (t13774, t13775, t13776, t13778, t13779, t13781, t13783) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1924::<F>(t550, t5658, t543, t3992, t2661, t5610, t9775, t1889, t9779, t828, t9954);
    (t13768, t13769, t13770, t13773, t13774, t13775, t13776, t13778, t13779, t13781, t13783)
}
