//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta324<F: Float>(t1146: F, t2439: F, t3361: F, t57: F, t268: F, t404: F, t7021: F, t1123: F, t2435: F) -> (F, F, F, F, F) {
        let (t12261, t12268, t12295, t12296, t12297) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1243::<F>(t1146, t2439, t3361, t57, t268, t404, t7021, t1123, t2435);
    (t12261, t12268, t12295, t12296, t12297)
}
