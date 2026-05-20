//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta375<F: Float>(t15191: F, t4628: F, t698: F, t15127: F, t15125: F, t3014: F, t4707: F, t15168: F, t4682: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15192, t15197, t15198, t15209, t15210, t15211, t15258, t15301, t15312, t15322, t15324, t15343) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1416::<F>(t15191, t4628, t698, t15127, t15125, t3014, t4707, t15168, t4682, t964);
    (t15192, t15197, t15198, t15209, t15210, t15211, t15258, t15301, t15312, t15322, t15324, t15343)
}
