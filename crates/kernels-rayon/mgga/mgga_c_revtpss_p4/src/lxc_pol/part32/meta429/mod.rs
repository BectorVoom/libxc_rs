//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1522;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1523;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1524;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1525;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta429(t1168: f64, t6487: f64, t1745: f64, t5142: f64, t6506: f64, t6503: f64, t3479: f64, t6502: f64, t5146: f64, t12472: f64, t6486: f64, t1130: f64, t6433: f64, t1151: f64, t16835: f64, t1733: f64, t5063: f64, t5105: f64, t12361: f64, t6439: f64, t3379: f64, t6471: f64, t12429: f64, t12470: f64, t17032: f64, t3452: f64, t3477: f64, t5147: f64, t1149: f64, t3384: f64, t3435: f64, t6470: f64, t3433: f64, t5104: f64, t5108: f64, t12230: f64, t6438: f64, t12227: f64, t1187: f64, t6519: f64, t1757: f64, t5180: f64, t6538: f64, t6535: f64, t3523: f64, t6534: f64, t5184: f64, t12555: f64, t6518: f64, t12486: f64, t12553: f64, t17097: f64, t17154: f64, t3496: f64, t3521: f64, t5163: f64, t5185: f64, t20545: f64, t20602: f64, t300: f64, t20568: f64, t20261: f64, t20263: f64, t20386: f64, t20388: f64, t20390: f64, t20393: f64, t20396: f64, t20399: f64, t20402: f64, t20404: f64, t20450: f64, t20452: f64, t20454: f64, t20471: f64, t20475: f64, t20477: f64, t5023: f64, t5501: f64, t5505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20606, t20609, t20612, t20615, t20619, t20622, t20626, t20629) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1522(t1168, t6487, t1745, t5142, t6506, t6503, t3479, t6502, t5146, t12472, t6486, t1130, t6433);
        let (t20631, t20633, t20635, t20637, t20639, t20640) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1523(t1151, t20629, t16835, t1733, t5063, t5105, t12361, t6439, t3379, t6471, t12429, t12470, t17032, t20606, t20609, t20612, t20615, t20619, t20622, t20626, t3452, t3477, t5147);
        let (t20643, t20647, t20650, t20654, t20659) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1524(t1149, t6471, t3384, t3435, t6470, t3433, t5104, t5108, t12230, t6438, t12227, t1187, t6519);
        let t20682 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1525(t1757, t5180, t1187, t6538, t6535, t3523, t6534, t5184, t12555, t6518, t12486, t12553, t17097, t17154, t20643, t20647, t20650, t20654, t20659, t3496, t3521, t5163, t5185);
        let (t20685, t20690, t20691) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1526(t20545, t20602, t20640, t20682, t300, t20568, t20261, t20263, t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20471, t20475, t20477, t5023, t5501, t5505);
    (t20631, t20633, t20635, t20637, t20639, t20643, t20647, t20650, t20654, t20685, t20690, t20691)
}
