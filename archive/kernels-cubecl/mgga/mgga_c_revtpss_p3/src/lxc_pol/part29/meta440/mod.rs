//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1651;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta440<F: Float>(t2311: F, t76: F, t10298: F, t38: F, t2248: F, t77: F, t84: F, t2247: F, t607: F, t1927: F, t644: F, t4144: F, t9593: F, t196: F, t197: F, t3821: F, t2394: F, t30: F, t2411: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25146, t25150, t25159, t25162) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1651::<F>(t2311, t76, t10298, t38, t2248, t77, t84, t2247, t607);
        let (t25163, t25177, t25188, t25198, t25207) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1652::<F>(t1927, t644, t4144, t9593, t196, t197, t3821, t2394, t30, t2411);
    (t25146, t25150, t25159, t25162, t25163, t25177, t25188, t25198, t25207)
}
