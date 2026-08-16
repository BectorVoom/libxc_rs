//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2593;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta649<F: Float>(t1757: F, t5180: F, t1187: F, t6538: F, t6535: F, t3523: F, t6534: F, t5184: F, t12555: F, t6518: F, t12486: F, t12553: F, t17097: F, t17154: F, t20643: F, t20647: F, t20650: F, t20654: F, t20659: F, t3496: F, t3521: F, t5163: F, t5185: F, t20545: F, t20602: F, t20640: F, t300: F, t20568: F, t20261: F, t20263: F, t20386: F, t20388: F, t20390: F, t20393: F, t20396: F, t20399: F, t20402: F, t20404: F, t20450: F, t20452: F, t20454: F, t20471: F, t20475: F, t20477: F, t5023: F, t5501: F, t5505: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20662, t20665, t20668, t20671, t20672, t20675, t20678, t20679, t20682) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2593::<F>(t1757, t5180, t1187, t6538, t6535, t3523, t6534, t5184, t12555, t6518, t12486, t12553, t17097, t17154, t20643, t20647, t20650, t20654, t20659, t3496, t3521, t5163, t5185);
        let (t20685, t20690, t20691) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2594::<F>(t20545, t20602, t20640, t20682, t300, t20568, t20261, t20263, t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20471, t20475, t20477, t5023, t5501, t5505);
    (t20662, t20665, t20668, t20671, t20672, t20675, t20678, t20679, t20685, t20690, t20691)
}
