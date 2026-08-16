//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta605<F: Float>(t12854: F, t29096: F, t11772: F, t26865: F, t3717: F, t13011: F, t7607: F, t12909: F, t26866: F, t12831: F, t13032: F, t26843: F) -> (F, F, F, F, F, F, F) {
        let (t97149, t97173, t97174, t97177, t97179, t97182, t97206) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2029::<F>(t12854, t29096, t11772, t26865, t3717, t13011, t7607, t12909, t26866, t12831, t13032, t26843);
    (t97149, t97173, t97174, t97177, t97179, t97182, t97206)
}
