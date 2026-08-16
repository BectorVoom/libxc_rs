//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2162;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2163;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2164;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2165;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2166;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2167;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta625(t2470: f64, t27278: f64, t7064: f64, t10073: f64, t25402: f64, t7056: f64, t7759: f64, t136: f64, t2457: f64, t7769: f64, t93377: f64, t4534: f64, t689: f64, t7014: f64, t27303: f64, t786: f64, t789: f64, t25296: f64, t27216: f64, t25407: f64, t27353: f64, t27357: f64, t51529: f64, t7775: f64, t7779: f64, t93126: f64, t93158: f64, t93161: f64, t93167: f64, t93175: f64, t93177: f64, t14991: f64, t93261: f64, t27213: f64, t92843: f64, t98815: f64, t1955: f64, t25309: f64, t27291: f64, t25431: f64, t25411: f64, t2453: f64, t27212: f64, t25301: f64, t25410: f64, t7774: f64, t93240: f64, t14662: f64, t1949: f64, t231: f64, t27350: f64, t27354: f64, t4423: f64, t7048: f64, t7070: f64, t7076: f64, t92917: f64, t93180: f64, t93184: f64, t93192: f64, t93195: f64, t1032: f64, t4469: f64, t867: f64, t7060: f64, t1559: f64, t2771: f64, t7760: f64, t2467: f64, t1579: f64, t2645: f64, t15030: f64, t25319: f64, t25391: f64, t25392: f64, t25426: f64, t27199: f64, t7053: f64, t93206: f64, t93207: f64, t93210: f64, t93224: f64, t93226: f64, t93228: f64, t93231: f64, t93349: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99201, t99202, t99206, t99211, t99212, t99216) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2162(t2470, t27278, t7064, t10073, t25402, t7056, t7759, t136, t2457, t7769, t93377, t4534, t689, t7014);
        let t99227 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2163(t27303, t786, t789, t25296, t27216, t25407, t27353, t27357, t51529, t7775, t7779, t93126, t93158, t93161, t93167, t93175, t93177, t99202, t99206, t99212, t99216);
        let (t99228, t99231, t99234, t99237, t99243, t99245) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2164(t14991, t93261, t25296, t27213, t92843, t98815, t1955, t25309, t27291, t689, t25431, t25411);
        let t99264 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2165(t2453, t27212, t25301, t25410, t7774, t93240, t14662, t1949, t231, t27350, t27354, t4423, t7048, t7070, t7076, t92917, t93180, t93184, t93192, t93195, t99228, t99231, t99234, t99237, t99243, t99245);
        let (t99270, t99271, t99274, t99277, t99287, t99289) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2166(t1032, t4469, t867, t786, t7060, t1559, t2771, t7760, t2467, t1579, t231, t2645);
        let t99295 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2167(t15030, t25319, t25391, t25392, t25426, t27199, t7053, t93206, t93207, t93210, t93224, t93226, t93228, t93231, t93349, t99274, t99277, t99287, t99289);
    (t99201, t99211, t99227, t99237, t99264, t99270, t99271, t99295)
}
