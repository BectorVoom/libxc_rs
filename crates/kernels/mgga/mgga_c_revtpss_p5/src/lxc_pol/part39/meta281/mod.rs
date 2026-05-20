//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1024;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta281<F: Float>(t10059: F, t4086: F, t543: F, t2782: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F, t1398: F, t1419: F, t4056: F, t555: F, t1432: F, t2470: F, t4107: F, t1433: F, t9288: F, t4066: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10066, t10069, t10070, t10073, t10074, t10079) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1024::<F>(t10059, t4086, t543, t2782, t123, t212, t2434, t4089, t138, t2438, t785, t1398, t1419);
        let (t10080, t10085, t10098, t10102, t10103) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1025::<F>(t10079, t2782, t4056, t555, t4086, t543, t1432, t2470, t4107, t1433, t9288, t4066, t72);
    (t10066, t10069, t10070, t10073, t10074, t10080, t10085, t10098, t10102, t10103)
}
