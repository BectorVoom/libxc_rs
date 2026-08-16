//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta30 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk207;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk208;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta30<F: Float>(t572: F, t573: F, t10: F, t2: F, t17: F, t16: F, t3: F, t15: F, t14: F, t11: F, t22: F, t21: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t575, t576, t578, t579, t580) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk207::<F>(t572, t573, t10, t2, t17, t16, t3);
        let (t582, t583) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk208::<F>(t15, t580, t14, t2);
        let (t584, t586, t587, t588) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk209::<F>(t11, t583, t22, t21, t3);
    (t575, t576, t578, t579, t580, t582, t583, t584, t586, t587, t588)
}
