//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta558<F: Float>(t1873: F, t94519: F, t94520: F, t94527: F, t94537: F, t94540: F, t26004: F, t5690: F, t13951: F, t2018: F, t807: F, t94565: F) -> (F, F, F, F, F, F, F, F) {
        let (t98260, t98263, t98264, t98267, t98268, t98269, t98281, t98283) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1877::<F>(t1873, t94519, t94520, t94527, t94537, t94540, t26004, t5690, t13951, t2018, t807, t94565);
    (t98260, t98263, t98264, t98267, t98268, t98269, t98281, t98283)
}
