//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1348;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1349;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1350;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta397<F: Float>(t3801: F, t6748: F, t1209: F, t6695: F, t460: F, t1214: F, t6587: F, t1211: F, t6744: F, t1277: F, t1294: F, t6573: F, t1774: F, t5245: F, t1210: F, t1215: F, t1295: F, t1775: F, t18037: F, t3561: F, t3567: F, t3572: F, t3732: F, t5225: F, t5237: F, t5251: F, t5417: F, t5429: F, t5498: F, t6580: F, t6745: F, t6702: F, t3737: F, t17974: F, t5422: F, t487: F, t6564: F, t1770: F, t1811: F, t1248: F, t1715: F, t3604: F, t17353: F, t12712: F, t6638: F, t13033: F, t13058: F, t17211: F, t17219: F, t17227: F, t17243: F, t17258: F, t17260: F, t17351: F, t17654: F, t5270: F, t5304: F, t5381: F, t6631: F, t6635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20692, t20697, t20700, t20703, t20704, t20710, t20714) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1348::<F>(t3801, t6748, t1209, t6695, t460, t1214, t6587, t1211, t6744, t1277, t1294, t6573);
        let (t20721, t20735) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1349::<F>(t1774, t5245, t1211, t1294, t6587, t1277, t1210, t1215, t1295, t1775, t18037, t20697, t20700, t20704, t20710, t20714, t3561, t3567, t3572, t3732, t5225, t5237, t5251, t5417, t5429, t5498, t6580, t6745);
        let (t20741, t20744, t20747, t20748, t20753, t20756, t20759) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1350::<F>(t1214, t6702, t3737, t17974, t5422, t6573, t1211, t487, t6564, t1770, t1811, t1294, t6744);
        let (t20760, t20782) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1351::<F>(t20759, t3737, t1248, t1715, t3604, t17353, t12712, t6638, t13033, t13058, t17211, t17219, t17227, t17243, t17258, t17260, t17351, t17654, t5270, t5304, t5381, t6631, t6635);
    (t20692, t20703, t20721, t20735, t20741, t20744, t20747, t20748, t20753, t20756, t20760, t20782)
}
