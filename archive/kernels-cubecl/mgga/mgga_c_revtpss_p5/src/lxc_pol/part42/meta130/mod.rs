//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta130<F: Float>(t421: F, t3356: F, t1156: F, t1160: F, t1159: F, t431: F, t426: F, t3413: F, t434: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3434, t3435, t3439, t3447, t3451, t3452, t3459, t3466, t3475, t3476, t3477, t3478) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk628::<F>(t421, t3356, t1156, t1160, t1159, t431, t426, t3413, t434);
    (t3434, t3435, t3439, t3447, t3451, t3452, t3459, t3466, t3475, t3476, t3477, t3478)
}
