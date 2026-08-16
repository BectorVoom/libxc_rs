//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta297<F: Float>(t2986: F, t960: F, t11132: F, t1034: F, t3154: F, t357: F, t1024: F, t3105: F, t905: F, t606: F, t1052: F, t360: F) -> (F, F, F, F, F, F, F, F) {
        let (t11554, t11560, t11574, t11627, t11631, t11656, t11661, t11670) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1062::<F>(t2986, t960, t11132, t1034, t3154, t357, t1024, t3105, t905, t606, t1052, t360);
    (t11554, t11560, t11574, t11627, t11631, t11656, t11661, t11670)
}
