//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1958;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1959;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1960;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1961;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta590<F: Float>(t1358: F, t212: F, t28888: F, t689: F, t25898: F, t8099: F, t94849: F, t26277: F, t97916: F, t97799: F, t2439: F, t785: F, t8085: F, t1364: F, t28905: F, t786: F, t102113: F, t102117: F, t102120: F, t102122: F, t26241: F, t26246: F, t27837: F, t8095: F, t94610: F, t96206: F, t96187: F, t97688: F, t28791: F, t25899: F, t25921: F, t26257: F, t26282: F, t26347: F, t28806: F, t4077: F, t543: F, t5658: F, t5775: F, t7295: F, t7301: F, t7506: F, t8094: F, t94656: F, t96210: F, t96211: F, t96218: F, t96222: F, t96226: F, t96230: F, t3923: F, t136: F, t2457: F, t94589: F, t26072: F, t28845: F, t28840: F, t686: F, t72: F, t25895: F, t2470: F, t28779: F, t1398: F, t1444: F, t1903: F, t25924: F, t26079: F, t26333: F, t26343: F, t28862: F, t4003: F, t4056: F, t7296: F, t96232: F, t96234: F, t96237: F, t96240: F) -> (F, F, F, F, F, F, F) {
        let (t102129, t102131, t102133, t102135, t102139) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1958::<F>(t1358, t212, t28888, t689, t25898, t8099, t94849, t26277, t97916, t97799, t2439, t785, t8085);
        let t102148 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1959::<F>(t1364, t28905, t786, t102113, t102117, t102120, t102122, t102129, t102131, t102133, t102135, t102139, t26241, t26246, t27837, t8095, t94610, t96206);
        let (t102165, t102175) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1960::<F>(t96187, t97688, t28791, t689, t25899, t25921, t26257, t26282, t26347, t27837, t28806, t4077, t543, t5658, t5775, t7295, t7301, t7506, t8094, t94656, t96210, t96211, t96218, t96222, t96226, t96230);
        let (t102185, t102204, t102205, t102213, t102215, t102217) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1961::<F>(t3923, t8085, t136, t2457, t8094, t94589, t26072, t28845, t28840, t686, t72, t25895);
        let (t102218, t102222) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1962::<F>(t2470, t28779, t25895, t102185, t102205, t102213, t102217, t1398, t1444, t1903, t25924, t26079, t26333, t26343, t27837, t28862, t28888, t4003, t4056, t543, t7295, t7296, t7301, t8085, t96232, t96234, t96237, t96240);
    (t102148, t102165, t102175, t102204, t102215, t102218, t102222)
}
