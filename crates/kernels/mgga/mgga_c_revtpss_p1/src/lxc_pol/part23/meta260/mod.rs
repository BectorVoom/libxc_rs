//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1456;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1457;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1458;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta260<F: Float>(t3994: F, t808: F, t9845: F, t521: F, t9342: F, t14: F, t588: F, t2496: F, t4038: F, t123: F, t1330: F, t2630: F, t2516: F, t676: F, t3869: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9846, t9847, t9854, t9855, t9856, t9857, t9858, t9860) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1456::<F>(t3994, t808, t9845, t521, t9342, t14, t588, t2496, t4038, t123, t1330);
        let (t9861, t9863) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1457::<F>(t2630, t9860, t2516, t676);
        let (t9865, t9866) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1458::<F>(t3869, t9863, t2496, t676);
    (t9846, t9847, t9854, t9855, t9856, t9857, t9858, t9860, t9861, t9863, t9865, t9866)
}
