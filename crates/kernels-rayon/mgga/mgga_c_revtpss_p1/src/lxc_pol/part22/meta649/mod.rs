//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2593;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta649(t1757: f64, t5180: f64, t1187: f64, t6538: f64, t6535: f64, t3523: f64, t6534: f64, t5184: f64, t12555: f64, t6518: f64, t12486: f64, t12553: f64, t17097: f64, t17154: f64, t20643: f64, t20647: f64, t20650: f64, t20654: f64, t20659: f64, t3496: f64, t3521: f64, t5163: f64, t5185: f64, t20545: f64, t20602: f64, t20640: f64, t300: f64, t20568: f64, t20261: f64, t20263: f64, t20386: f64, t20388: f64, t20390: f64, t20393: f64, t20396: f64, t20399: f64, t20402: f64, t20404: f64, t20450: f64, t20452: f64, t20454: f64, t20471: f64, t20475: f64, t20477: f64, t5023: f64, t5501: f64, t5505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20662, t20665, t20668, t20671, t20672, t20675, t20678, t20679, t20682) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2593(t1757, t5180, t1187, t6538, t6535, t3523, t6534, t5184, t12555, t6518, t12486, t12553, t17097, t17154, t20643, t20647, t20650, t20654, t20659, t3496, t3521, t5163, t5185);
        let (t20685, t20690, t20691) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2594(t20545, t20602, t20640, t20682, t300, t20568, t20261, t20263, t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20471, t20475, t20477, t5023, t5501, t5505);
    (t20662, t20665, t20668, t20671, t20672, t20675, t20678, t20679, t20685, t20690, t20691)
}
