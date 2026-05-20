//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta599<F: Float>(t5966: F, t605: F, t27375: F, t98658: F, t1468: F, t4343: F, t5962: F, t6075: F, t775: F, t25207: F, t1583: F, t580: F, t98631: F) -> (F, F, F, F, F, F, F) {
        let (t105902, t105906, t105909, t105919, t105923, t105924, t105928) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1934::<F>(t5966, t605, t27375, t98658, t1468, t4343, t5962, t6075, t775, t25207, t1583, t580, t98631);
    (t105902, t105906, t105909, t105919, t105923, t105924, t105928)
}
