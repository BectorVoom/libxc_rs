//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2362;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta547<F: Float>(t17687: F, t2251: F, t5351: F, t12787: F, t1285: F, t12865: F, t372: F, t5302: F, t4181: F, t5405: F) -> (F, F, F, F, F, F) {
        let (t17688, t17689, t17690, t17693) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2362::<F>(t17687, t2251, t5351, t12787, t1285, t12865);
        let (t17694, t17695) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2363::<F>(t372, t5302, t4181, t5405);
    (t17688, t17689, t17690, t17693, t17694, t17695)
}
