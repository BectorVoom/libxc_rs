//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta448<F: Float>(t14365: F, t25759: F, t1113: F, t775: F, t2430: F, t33: F, t2408: F, t890: F, t2832: F, t4135: F, t4147: F, t112: F, t239: F) -> (F, F, F, F, F, F, F, F) {
        let (t25760, t25763, t25767, t25778, t25781, t25784, t25802, t25821) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1677::<F>(t14365, t25759, t1113, t775, t2430, t33, t2408, t890, t2832, t4135, t4147, t112, t239);
    (t25760, t25763, t25767, t25778, t25781, t25784, t25802, t25821)
}
