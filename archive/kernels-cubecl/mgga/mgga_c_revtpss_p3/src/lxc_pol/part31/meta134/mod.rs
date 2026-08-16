//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk733;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta134<F: Float>(t1126: F, t1130: F, t1129: F, t418: F, t408: F, t406: F, t409: F, t3356: F, t281: F, t2902: F, t414: F, t1146: F, t698: F, t1224: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3379, t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3415) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk733::<F>(t1126, t1130, t1129, t418, t408, t406, t409, t3356, t281, t2902, t414, t1146, t698);
        let t3417 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk734::<F>(t1224, t240);
    (t3379, t3383, t3384, t3390, t3394, t3402, t3407, t3413, t3414, t3415, t3417)
}
