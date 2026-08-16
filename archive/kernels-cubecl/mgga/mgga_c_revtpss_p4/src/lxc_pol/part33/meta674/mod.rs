//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta674<F: Float>(t1936: F, t21658: F, t651: F, t18245: F, t7003: F, t1518: F, t4245: F, t1937: F, t1501: F, t4292: F, t30138: F, t6993: F) -> (F, F, F, F, F, F, F) {
        let (t109147, t109149, t109150, t109152, t109153, t109155, t109157) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2205::<F>(t1936, t21658, t651, t18245, t7003, t1518, t4245, t1937, t1501, t4292, t30138, t6993);
    (t109147, t109149, t109150, t109152, t109153, t109155, t109157)
}
