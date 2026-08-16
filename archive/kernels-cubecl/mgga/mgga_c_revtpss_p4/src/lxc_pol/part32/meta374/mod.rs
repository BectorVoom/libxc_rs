//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1330;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta374<F: Float>(t1011: F, t16219: F, t15688: F, t3299: F, t1678: F, t3057: F, t4930: F, t994: F, t3046: F, t379: F, t1078: F, t1651: F) -> (F, F, F, F, F, F, F) {
        let (t16220, t16226, t16284, t16302, t16305, t16312, t16313) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1330::<F>(t1011, t16219, t15688, t3299, t1678, t3057, t4930, t994, t3046, t379, t1078, t1651);
    (t16220, t16226, t16284, t16302, t16305, t16312, t16313)
}
