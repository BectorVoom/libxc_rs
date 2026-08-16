//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta397 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1345;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1346;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1347;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1348;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta397<F: Float>(t1168: F, t6487: F, t1745: F, t5142: F, t6506: F, t6503: F, t3479: F, t6502: F, t5146: F, t12472: F, t6486: F, t1130: F, t6433: F, t1151: F, t16835: F, t1733: F, t5063: F, t5105: F, t12361: F, t6439: F, t3379: F, t6471: F, t12429: F, t12470: F, t17032: F, t3452: F, t3477: F, t5147: F, t1149: F, t3384: F, t3435: F, t6470: F, t3433: F, t5104: F, t5108: F, t12230: F, t6438: F, t12227: F, t1187: F, t6519: F, t1757: F, t5180: F, t6538: F, t6535: F, t3523: F, t6534: F, t5184: F, t12555: F, t6518: F, t12486: F, t12553: F, t17097: F, t17154: F, t3496: F, t3521: F, t5163: F, t5185: F, t20545: F, t20602: F, t300: F, t20568: F, t20261: F, t20263: F, t20386: F, t20388: F, t20390: F, t20393: F, t20396: F, t20399: F, t20402: F, t20404: F, t20450: F, t20452: F, t20454: F, t20471: F, t20475: F, t20477: F, t5023: F, t5501: F, t5505: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20606, t20609, t20612, t20615, t20619, t20622, t20626, t20629) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1345::<F>(t1168, t6487, t1745, t5142, t6506, t6503, t3479, t6502, t5146, t12472, t6486, t1130, t6433);
        let (t20631, t20633, t20635, t20637, t20639, t20640) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1346::<F>(t1151, t20629, t16835, t1733, t5063, t5105, t12361, t6439, t3379, t6471, t12429, t12470, t17032, t20606, t20609, t20612, t20615, t20619, t20622, t20626, t3452, t3477, t5147);
        let (t20643, t20647, t20650, t20654, t20659) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1347::<F>(t1149, t6471, t3384, t3435, t6470, t3433, t5104, t5108, t12230, t6438, t12227, t1187, t6519);
        let t20682 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1348::<F>(t1757, t5180, t1187, t6538, t6535, t3523, t6534, t5184, t12555, t6518, t12486, t12553, t17097, t17154, t20643, t20647, t20650, t20654, t20659, t3496, t3521, t5163, t5185);
        let (t20685, t20690, t20691) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1349::<F>(t20545, t20602, t20640, t20682, t300, t20568, t20261, t20263, t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20471, t20475, t20477, t5023, t5501, t5505);
    (t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20685, t20690, t20691)
}
