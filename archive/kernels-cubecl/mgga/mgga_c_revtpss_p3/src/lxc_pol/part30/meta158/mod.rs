//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk814;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta158<F: Float>(t3363: F, t3417: F, t141: F, t1145: F, t3368: F, t3372: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3402: F, t3408: F, t3410: F, t3414: F, t3415: F, t1150: F, t1131: F, t1129: F, t408: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3418, t3419, t3421, t3422, t3424, t3425, t3427) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk814::<F>(t3363, t3417, t141, t1145, t3368, t3372, t3358, t3365, t3370, t3374, t3392, t3400, t3402, t3408, t3410, t3414, t3415);
        let (t3428, t3430, t3431, t3432, t3433) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk815::<F>(t1150, t3427, t1131, t1129, t408);
    (t3418, t3419, t3421, t3422, t3424, t3425, t3427, t3428, t3430, t3431, t3432, t3433)
}
