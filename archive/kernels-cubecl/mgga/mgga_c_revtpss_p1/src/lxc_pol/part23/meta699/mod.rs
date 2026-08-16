//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2448;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta699<F: Float>(t1331: F, t9855: F, t3825: F, t9586: F, t1333: F, t9342: F, t521: F, t583: F, t596: F, t525: F, t9603: F, t527: F, t9615: F, t1340: F, t40165: F, t268: F, t520: F, t39768: F, t190: F, t22: F, t519: F, t39762: F, t1317: F, t9545: F, t40129: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47007, t47011, t47013, t47019, t47025, t47040) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2448::<F>(t1331, t9855, t3825, t9586, t1333, t9342, t521, t583, t596, t525, t9603, t527, t9615);
        let (t47059, t47067, t47070, t47072, t47073, t47076) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2449::<F>(t1340, t40165, t268, t520, t39768, t190, t22, t519, t39762, t1317, t9545, t40129);
    (t47007, t47011, t47013, t47019, t47025, t47040, t47059, t47067, t47070, t47072, t47073, t47076)
}
