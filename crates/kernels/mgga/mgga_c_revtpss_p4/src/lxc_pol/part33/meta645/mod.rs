//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta645<F: Float>(t29010: F, t3704: F, t17720: F, t7624: F, t15904: F, t26865: F, t13127: F, t17400: F, t26866: F, t1802: F, t3089: F, t3717: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
        let (t104689, t104691, t104695, t104696, t104703, t104706, t104707, t104708) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2094::<F>(t29010, t3704, t17720, t7624, t15904, t26865, t13127, t17400, t26866, t1802, t3089, t3717, sigma2);
    (t104689, t104691, t104695, t104696, t104703, t104706, t104707, t104708)
}
