//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1224;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta273<F: Float>(t1936: F, t4248: F, t1518: F, t93: F, t1312: F, t7741: F, t1847: F, t196: F, t197: F) -> (F, F, F, F, F, F) {
        let (t7888, t7889) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1224::<F>(t1936, t4248, t1518, t93);
        let (t7891, t7893, t7897, t7898) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1225::<F>(t1936, t7889, t1312, t7741, t1847, t196, t197);
    (t7888, t7889, t7891, t7893, t7897, t7898)
}
