//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta570<F: Float>(t138: F, t785: F, t9302: F, t2452: F, t9720: F, t11006: F, t256: F, t10115: F, t251: F, t2410: F, t11238: F, t196: F) -> (F, F, F, F, F, F) {
        let (t40270, t40688, t41077, t41117, t41154, t42859) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1979::<F>(t138, t785, t9302, t2452, t9720, t11006, t256, t10115, t251, t2410, t11238, t196);
    (t40270, t40688, t41077, t41117, t41154, t42859)
}
