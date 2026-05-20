//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta305<F: Float>(t3475: F, t431: F, t426: F, t12295: F, t12351: F, t1159: F, t3478: F, t434: F, t1175: F, t3520: F, t3519: F, t444: F) -> (F, F, F, F, F, F, F) {
        let (t12429, t12459, t12460, t12470, t12472, t12481, t12485) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1073::<F>(t3475, t431, t426, t12295, t12351, t1159, t3478, t434, t1175, t3520, t3519, t444);
    (t12429, t12459, t12460, t12470, t12472, t12481, t12485)
}
