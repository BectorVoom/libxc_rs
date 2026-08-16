//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta394<F: Float>(t17240: F, t5052: F, t1222: F, t3636: F, t5391: F, t5381: F, t1803: F, t3666: F, t1208: F, t5215: F, t225: F, t480: F) -> (F, F, F, F, F, F, F) {
        let (t17243, t17258, t17260, t17283, t17288, t17289, t17290) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1444::<F>(t17240, t5052, t1222, t3636, t5391, t5381, t1803, t3666, t1208, t5215, t225, t480);
    (t17243, t17258, t17260, t17283, t17288, t17289, t17290)
}
