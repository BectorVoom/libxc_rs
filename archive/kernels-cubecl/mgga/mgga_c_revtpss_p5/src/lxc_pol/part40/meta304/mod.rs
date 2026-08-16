//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1070;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1071;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta304<F: Float>(t11043: F, t786: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t2410: F, t261: F, t2832: F, t892: F, t2408: F, t2411: F, t3335: F, t389: F, t1077: F, t225: F, t1071: F, t3046: F, t268: F, t271: F, t7021: F, t2435: F, t907: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11044, t11045, t11051, t11064, t11075, t11084) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1070::<F>(t11043, t786, t2467, t2828, t676, t123, t2465, t2410, t261, t2832, t892, t2408, t2411);
        let (t11108, t11121, t11128, t11132) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1071::<F>(t3335, t389, t1077, t225, t1071, t3046, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1072::<F>(t11132, t2435, t907);
    (t11044, t11045, t11051, t11064, t11075, t11084, t11108, t11121, t11128, t11132, t11133, t11134)
}
