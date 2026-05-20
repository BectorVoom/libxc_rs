//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta562 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2129;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2130;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta562<F: Float>(t1497: F, t5816: F, t5872: F, t1927: F, t5825: F, t1486: F, t5819: F, t22603: F, t30: F, t33: F, zeta_threshold: F, t36: F, t70: F, t5826: F, t1470: F, t5854: F, t1469: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22656, t22659, t22662, t22665, t22670) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2129::<F>(t1497, t5816, t5872, t1927, t5825, t1486, t5819, t22603);
        let t22671 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2130::<F>(t30, t33, t22670, zeta_threshold);
        let (t22672, t22673, t22676, t22681, t22688) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2131::<F>(t22671, t36, t70, t1486, t5826, t1470, t5854, t1469, t5819);
    (t22656, t22659, t22662, t22665, t22670, t22671, t22672, t22673, t22676, t22681, t22688)
}
