//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk715;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta157<F: Float>(t1169: F, t3471: F, t1159: F, t426: F, t434: F, t3453: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t448: F, t1175: F, t1179: F, t1178: F, t444: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3472, t3475, t3476, t3477, t3478, t3479, t3480, t3483, t3488, t3489) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk715::<F>(t1169, t3471, t1159, t426, t434, t3453, t3356, t3358, t3365, t3370, t3374, t448);
        let (t3491, t3495) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk716::<F>(t1175, t1179, t1178, t444);
    (t3472, t3475, t3476, t3477, t3478, t3479, t3480, t3483, t3488, t3489, t3491, t3495)
}
