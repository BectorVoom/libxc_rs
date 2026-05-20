//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta599<F: Float>(t1269: F, t7642: F, t8945: F, t1243: F, t26884: F, t12941: F, t7618: F, t13068: F, t7617: F, t26873: F, t3704: F, t12959: F, t26880: F) -> (F, F, F, F, F, F) {
        let (t97082, t97095, t97112, t97120, t97125, t97136) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2060::<F>(t1269, t7642, t8945, t1243, t26884, t12941, t7618, t13068, t7617, t26873, t3704, t12959, t26880);
    (t97082, t97095, t97112, t97120, t97125, t97136)
}
