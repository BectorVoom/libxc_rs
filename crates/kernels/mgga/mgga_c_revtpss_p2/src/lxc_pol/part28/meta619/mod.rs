//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2177;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2178;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2179;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2180;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2181;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta619<F: Float>(t2470: F, t27278: F, t7064: F, t10073: F, t25402: F, t7056: F, t7759: F, t136: F, t2457: F, t7769: F, t93377: F, t4534: F, t689: F, t7014: F, t27303: F, t786: F, t789: F, t25296: F, t27216: F, t25407: F, t27353: F, t27357: F, t51529: F, t7775: F, t7779: F, t93126: F, t93158: F, t93161: F, t93167: F, t93175: F, t93177: F, t14991: F, t93261: F, t27213: F, t92843: F, t98815: F, t1955: F, t25309: F, t27291: F, t25431: F, t25411: F, t2453: F, t27212: F, t25301: F, t25410: F, t7774: F, t93240: F, t14662: F, t1949: F, t231: F, t27350: F, t27354: F, t4423: F, t7048: F, t7070: F, t7076: F, t92917: F, t93180: F, t93184: F, t93192: F, t93195: F, t1032: F, t4469: F, t867: F, t7060: F, t1559: F, t2771: F, t7760: F, t2467: F, t1579: F, t2645: F, t15030: F, t25319: F, t25391: F, t25392: F, t25426: F, t27199: F, t7053: F, t93206: F, t93207: F, t93210: F, t93224: F, t93226: F, t93228: F, t93231: F, t93349: F) -> (F, F, F, F, F, F, F, F) {
        let (t99201, t99202, t99206, t99211, t99212, t99216) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2177::<F>(t2470, t27278, t7064, t10073, t25402, t7056, t7759, t136, t2457, t7769, t93377, t4534, t689, t7014);
        let t99227 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2178::<F>(t27303, t786, t789, t25296, t27216, t25407, t27353, t27357, t51529, t7775, t7779, t93126, t93158, t93161, t93167, t93175, t93177, t99202, t99206, t99212, t99216);
        let (t99228, t99231, t99234, t99237, t99243, t99245) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2179::<F>(t14991, t93261, t25296, t27213, t92843, t98815, t1955, t25309, t27291, t689, t25431, t25411);
        let t99264 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2180::<F>(t2453, t27212, t25301, t25410, t7774, t93240, t14662, t1949, t231, t27350, t27354, t4423, t7048, t7070, t7076, t92917, t93180, t93184, t93192, t93195, t99228, t99231, t99234, t99237, t99243, t99245);
        let (t99270, t99271, t99274, t99277, t99287, t99289) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2181::<F>(t1032, t4469, t867, t786, t7060, t1559, t2771, t7760, t2467, t1579, t231, t2645);
        let t99295 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2182::<F>(t15030, t25319, t25391, t25392, t25426, t27199, t7053, t93206, t93207, t93210, t93224, t93226, t93228, t93231, t93349, t99274, t99277, t99287, t99289);
    (t99201, t99211, t99227, t99237, t99264, t99270, t99271, t99295)
}
