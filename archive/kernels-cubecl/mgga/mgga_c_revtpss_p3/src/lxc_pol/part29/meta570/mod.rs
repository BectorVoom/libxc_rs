//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta570<F: Float>(t27375: F, t890: F, t27383: F, t1583: F, t2832: F, t30: F, t41154: F, t2408: F, t1468: F, t2394: F, t14495: F, t689: F) -> (F, F, F, F, F, F, F, F) {
        let (t98767, t98768, t98779, t98780, t98786, t98787, t98793, t98801) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1917::<F>(t27375, t890, t27383, t1583, t2832, t30, t41154, t2408, t1468, t2394, t14495, t689);
    (t98767, t98768, t98779, t98780, t98786, t98787, t98793, t98801)
}
