//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta352<F: Float>(t221: F, t5627: F, t9921: F, t3978: F, t2619: F, t5635: F, t1398: F, t1882: F, t13848: F, t3938: F, t9818: F, t9816: F) -> (F, F, F, F, F, F, F) {
        let (t13877, t13878, t13880, t13887, t13926, t13941, t13943) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1369::<F>(t221, t5627, t9921, t3978, t2619, t5635, t1398, t1882, t13848, t3938, t9818, t9816);
    (t13877, t13878, t13880, t13887, t13926, t13941, t13943)
}
