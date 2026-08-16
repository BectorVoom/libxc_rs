//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk871;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta218<F: Float>(t1015: F, t4186: F, t1012: F, t3147: F, t72: F, t3088: F, t3299: F, t1668: F, t3153: F) -> (F, F, F, F, F, F) {
        let (t4886, t4887, t4890, t4891) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk871::<F>(t1015, t4186, t1012, t3147, t72, t3088);
        let (t4892, t4893) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk872::<F>(t3299, t4891, t1668, t3153);
    (t4886, t4887, t4890, t4891, t4892, t4893)
}
