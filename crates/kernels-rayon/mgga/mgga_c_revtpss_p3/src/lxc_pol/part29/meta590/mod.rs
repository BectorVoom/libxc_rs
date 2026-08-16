//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1958;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1959;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1960;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1961;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta590(t1358: f64, t212: f64, t28888: f64, t689: f64, t25898: f64, t8099: f64, t94849: f64, t26277: f64, t97916: f64, t97799: f64, t2439: f64, t785: f64, t8085: f64, t1364: f64, t28905: f64, t786: f64, t102113: f64, t102117: f64, t102120: f64, t102122: f64, t26241: f64, t26246: f64, t27837: f64, t8095: f64, t94610: f64, t96206: f64, t96187: f64, t97688: f64, t28791: f64, t25899: f64, t25921: f64, t26257: f64, t26282: f64, t26347: f64, t28806: f64, t4077: f64, t543: f64, t5658: f64, t5775: f64, t7295: f64, t7301: f64, t7506: f64, t8094: f64, t94656: f64, t96210: f64, t96211: f64, t96218: f64, t96222: f64, t96226: f64, t96230: f64, t3923: f64, t136: f64, t2457: f64, t94589: f64, t26072: f64, t28845: f64, t28840: f64, t686: f64, t72: f64, t25895: f64, t2470: f64, t28779: f64, t1398: f64, t1444: f64, t1903: f64, t25924: f64, t26079: f64, t26333: f64, t26343: f64, t28862: f64, t4003: f64, t4056: f64, t7296: f64, t96232: f64, t96234: f64, t96237: f64, t96240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t102129, t102131, t102133, t102135, t102139) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1958(t1358, t212, t28888, t689, t25898, t8099, t94849, t26277, t97916, t97799, t2439, t785, t8085);
        let t102148 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1959(t1364, t28905, t786, t102113, t102117, t102120, t102122, t102129, t102131, t102133, t102135, t102139, t26241, t26246, t27837, t8095, t94610, t96206);
        let (t102165, t102175) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1960(t96187, t97688, t28791, t689, t25899, t25921, t26257, t26282, t26347, t27837, t28806, t4077, t543, t5658, t5775, t7295, t7301, t7506, t8094, t94656, t96210, t96211, t96218, t96222, t96226, t96230);
        let (t102185, t102204, t102205, t102213, t102215, t102217) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1961(t3923, t8085, t136, t2457, t8094, t94589, t26072, t28845, t28840, t686, t72, t25895);
        let (t102218, t102222) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1962(t2470, t28779, t25895, t102185, t102205, t102213, t102217, t1398, t1444, t1903, t25924, t26079, t26333, t26343, t27837, t28862, t28888, t4003, t4056, t543, t7295, t7296, t7301, t8085, t96232, t96234, t96237, t96240);
    (t102148, t102165, t102175, t102204, t102215, t102218, t102222)
}
