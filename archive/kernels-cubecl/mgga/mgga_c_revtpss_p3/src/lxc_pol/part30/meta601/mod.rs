//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta601<F: Float>(t11772: F, t26865: F, t3717: F, t13011: F, t7607: F, t12909: F, t26866: F, t12831: F, t12917: F, t26870: F, t26827: F, t3678: F) -> (F, F, F, F, F, F, F) {
        let (t97173, t97174, t97177, t97179, t97182, t97187, t97191) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2062::<F>(t11772, t26865, t3717, t13011, t7607, t12909, t26866, t12831, t12917, t26870, t26827, t3678);
    (t97173, t97174, t97177, t97179, t97182, t97187, t97191)
}
