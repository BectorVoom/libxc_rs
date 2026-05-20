//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1490;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta413<F: Float>(t2349: F, t43: F, t10227: F, t96: F, t100: F, t613: F, t10199: F, t2175: F, t2289: F, t8264: F, t31051: F, t625: F, t31027: F, t31044: F, t2184: F, t4168: F, t31127: F, t571: F, t2192: F, t4153: F, t1455: F, t8302: F, t116: F, t31066: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t116942, t116946, t116957, t116968, t116969, t116971) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1490::<F>(t2349, t43, t10227, t96, t100, t613, t10199, t2175, t2289, t8264, t31051, t625);
        let (t116995, t117090, t117095, t117097, t117099, t117103) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1491::<F>(t31027, t31044, t2184, t4168, t31127, t571, t2192, t4153, t1455, t8302, t116, t31066);
    (t116942, t116946, t116957, t116968, t116969, t116971, t116995, t117090, t117095, t117097, t117099, t117103)
}
