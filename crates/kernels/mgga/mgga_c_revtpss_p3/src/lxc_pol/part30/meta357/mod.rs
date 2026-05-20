//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta357<F: Float>(t159: F, t3617: F, t409: F, t416: F, t406: F, t12295: F, t11335: F, t281: F, t414: F, t1126: F, t3383: F, t1160: F, t3444: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12305, t12327, t12331, t12349, t12351, t12352, t12361, t12367, t12382, t12397, t12418) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1382::<F>(t159, t3617, t409, t416, t406, t12295, t11335, t281, t414, t1126, t3383, t1160, t3444);
    (t12305, t12327, t12331, t12349, t12351, t12352, t12361, t12367, t12382, t12397, t12418)
}
