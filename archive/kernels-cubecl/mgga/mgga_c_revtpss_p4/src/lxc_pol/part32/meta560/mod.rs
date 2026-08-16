//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta560<F: Float>(t1468: F, t2411: F, t30: F, t41154: F, t14495: F, t689: F, t14587: F, t27312: F, t1568: F, t7063: F, t25410: F, t25304: F, t27212: F) -> (F, F, F, F, F, F, F, F) {
        let (t98658, t98785, t98801, t98809, t98815, t98848, t98849, t98867) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1879::<F>(t1468, t2411, t30, t41154, t14495, t689, t14587, t27312, t1568, t7063, t25410, t25304, t27212);
    (t98658, t98785, t98801, t98809, t98815, t98848, t98849, t98867)
}
