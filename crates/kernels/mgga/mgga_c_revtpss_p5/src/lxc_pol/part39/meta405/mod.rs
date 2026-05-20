//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta405<F: Float>(t1453: F, t8362: F, t31027: F, t8355: F, t28036: F, t8259: F, t1513: F, t31039: F, t658: F, t8268: F, t4287: F, t31032: F, t8358: F) -> (F, F, F, F, F, F, F, F) {
        let (t31248, t31259, t31261, t31264, t31267, t31268, t31271, t31274) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1479::<F>(t1453, t8362, t31027, t8355, t28036, t8259, t1513, t31039, t658, t8268, t4287, t31032, t8358);
    (t31248, t31259, t31261, t31264, t31267, t31268, t31271, t31274)
}
