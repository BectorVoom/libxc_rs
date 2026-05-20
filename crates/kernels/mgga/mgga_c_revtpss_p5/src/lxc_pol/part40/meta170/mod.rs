//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta170<F: Float>(t1280: F, t3568: F, t1284: F, t487: F, t1209: F, t1287: F, t3721: F, t1269: F, t473: F, t1214: F, t3584: F, t3140: F, t3596: F) -> (F, F, F, F, F, F, F, F) {
        let (t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk756::<F>(t1280, t3568, t1284, t487, t1209, t1287, t3721, t1269, t473, t1214, t3584, t3140, t3596);
    (t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766)
}
