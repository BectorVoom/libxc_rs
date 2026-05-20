//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1755;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta460<F: Float>(t2315: F, t84: F, t77: F, t2251: F, t603: F, t2259: F, t239: F, t2311: F, t76: F, t10298: F, t38: F, t2248: F, t2247: F, t607: F) -> (F, F, F, F, F, F, F, F) {
        let (t25114, t25117, t25120, t25137, t25146, t25150, t25159) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1755::<F>(t2315, t84, t77, t2251, t603, t2259, t239, t2311, t76, t10298, t38, t2248);
        let t25162 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1756::<F>(t2247, t607);
    (t25114, t25117, t25120, t25137, t25146, t25150, t25159, t25162)
}
