//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta196<F: Float>(t1121: F, t1263: F, t1214: F, t1469: F, t1042: F, t3362: F, t3617: F, t4181: F, t1012: F, t1224: F, t5052: F, t3698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5296, t5297, t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk794::<F>(t1121, t1263, t1214, t1469, t1042, t3362, t3617, t4181, t1012, t1224, t5052, t3698);
    (t5296, t5297, t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312)
}
