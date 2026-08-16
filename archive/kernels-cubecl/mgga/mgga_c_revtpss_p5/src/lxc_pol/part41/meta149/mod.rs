//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta149 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk674;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta149<F: Float>(t1398: F, t555: F, t4086: F, t543: F, t2782: F, t1419: F, t545: F, t869: F, t689: F, t136: F, t2457: F, t3964: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4089, t4090, t4092, t4093, t4094, t4096, t4099) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk674::<F>(t1398, t555, t4086, t543, t2782, t1419, t545, t869, t689, t136, t2457, t3964);
        let (t4100, t4101) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk675::<F>(t4086, t555, t786);
    (t4089, t4090, t4092, t4093, t4094, t4096, t4099, t4100, t4101)
}
