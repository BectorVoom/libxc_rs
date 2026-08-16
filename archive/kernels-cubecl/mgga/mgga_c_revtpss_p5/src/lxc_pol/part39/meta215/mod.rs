//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta215<F: Float>(t4181: F, t4801: F, t1042: F, t2852: F, t3181: F, t1592: F, t3109: F, t247: F, t1063: F, t1670: F, t3172: F, t1041: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4802, t4803, t4806, t4807, t4808, t4817, t4818, t4820, t4821) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk862::<F>(t4181, t4801, t1042, t2852, t3181, t1592, t3109, t247, t1063, t1670, t3172, t1041);
    (t4802, t4803, t4806, t4807, t4808, t4817, t4818, t4820, t4821)
}
