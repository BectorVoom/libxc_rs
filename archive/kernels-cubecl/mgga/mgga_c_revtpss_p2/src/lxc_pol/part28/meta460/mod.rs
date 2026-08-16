//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1757;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta460<F: Float>(t2242: F, t607: F, t38: F, t6972: F, t2247: F, t640: F, t644: F, t77: F, t2315: F, t84: F, t2251: F, t603: F, t2259: F, t48: F, t613: F, t2275: F, t43: F, t239: F, t2258: F, t2269: F, t49: F, t606: F, t6968: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25102, t25105, t25106, t25110, t25114, t25117) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1757::<F>(t2242, t607, t38, t6972, t2247, t640, t644, t77, t2315, t84, t2251, t603);
        let (t25120, t25129, t25132, t25137, t25138) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1758::<F>(t2259, t603, t48, t613, t2275, t43, t239, t2251, t2258, t2269, t49, t606, t6968);
    (t25102, t25105, t25106, t25110, t25114, t25117, t25120, t25129, t25132, t25137, t25138)
}
