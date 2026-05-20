//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta622<F: Float>(t30: F, t41154: F, t1957: F, t25392: F, t14495: F, t689: F, t25372: F, t25386: F, t27357: F, t14587: F, t27312: F, t92838: F) -> (F, F, F, F, F, F, F) {
        let (t98785, t98803, t98806, t98811, t98814, t98815, t98817) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2061::<F>(t30, t41154, t1957, t25392, t14495, t689, t25372, t25386, t27357, t14587, t27312, t92838);
    (t98785, t98803, t98806, t98811, t98814, t98815, t98817)
}
