//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta742 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2613;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta742<F: Float>(t48089: F, t221: F, t9817: F, t13792: F, t13845: F, t1882: F, t9994: F, t13793: F, t13999: F, t1868: F, t3923: F, t1353: F, t13783: F, t13789: F, t13790: F, t13791: F, t13804: F, t1398: F, t21990: F, t3889: F, t46592: F, t46596: F, t46598: F, t46600: F, t46602: F, t46607: F, t46613: F, t46618: F, t46620: F, t46622: F, t46633: F, t46641: F, t5671: F, t9835: F, t13872: F, t3978: F, t9921: F, t1320: F, t13632: F, t13672: F, t1317: F, t13680: F, t3860: F, t5567: F, t46960: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48090, t48100, t48105, t48113, t48129) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2613::<F>(t48089, t221, t9817, t13792, t13845, t1882, t9994, t13793, t13999, t1868, t3923, t1353, t13783, t13789, t13790, t13791, t13804, t1398, t21990, t3889, t46592, t46596, t46598, t46600, t46602, t46607, t46613, t46618, t46620, t46622, t46633, t46641, t5671, t9835);
        let (t48143, t48153, t48155, t48157, t48159, t48160) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2614::<F>(t13872, t221, t3978, t9921, t1320, t13632, t13672, t1317, t13680, t3860, t5567, t46960);
    (t48090, t48100, t48105, t48113, t48129, t48143, t48153, t48155, t48157, t48159, t48160)
}
