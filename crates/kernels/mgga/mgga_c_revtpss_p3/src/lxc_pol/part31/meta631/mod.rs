//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta631<F: Float>(t12167: F, t99984: F, t12078: F, t25516: F, t4954: F, t15752: F, t27498: F, t15734: F, t25522: F, t15816: F, t7121: F, t15794: F, t25580: F) -> (F, F, F, F, F, F, F) {
        let (t100138, t100141, t100146, t100160, t100166, t100168, t100186) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2085::<F>(t12167, t99984, t12078, t25516, t4954, t15752, t27498, t15734, t25522, t15816, t7121, t15794, t25580);
    (t100138, t100141, t100146, t100160, t100166, t100168, t100186)
}
