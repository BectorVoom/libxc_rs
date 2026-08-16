//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta312<F: Float>(t11132: F, t2942: F, t941: F, t2986: F, t960: F, t2979: F, t300: F, t1034: F, t3154: F, t357: F, t3129: F, t3172: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11534, t11548, t11554, t11560, t11574, t11591, t11627, t11631, t11643) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1084::<F>(t11132, t2942, t941, t2986, t960, t2979, t300, t1034, t3154, t357, t3129, t3172);
    (t11534, t11548, t11554, t11560, t11574, t11591, t11627, t11631, t11643)
}
