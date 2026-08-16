//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta643<F: Float>(t17288: F, t2142: F, t5216: F, t1209: F, t2143: F, t26852: F, t5378: F, t29083: F, t3636: F, t1234: F, t29082: F, t17620: F, t26870: F) -> (F, F, F, F, F, F, F) {
        let (t104521, t104524, t104549, t104624, t104626, t104636, t104640) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2092::<F>(t17288, t2142, t5216, t1209, t2143, t26852, t5378, t29083, t3636, t1234, t29082, t17620, t26870);
    (t104521, t104524, t104549, t104624, t104626, t104636, t104640)
}
