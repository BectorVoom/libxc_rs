//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1347;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1348;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1349;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta332<F: Float>(t2832: F, t892: F, t2408: F, t2411: F, t3335: F, t389: F, t1077: F, t225: F, t1071: F, t3046: F, t268: F, t271: F, t7021: F, t2435: F, t907: F, t2854: F, t689: F, t2859: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11075, t11084, t11108, t11119, t11120, t11121, t11128, t11132) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1347::<F>(t2832, t892, t2408, t2411, t3335, t389, t1077, t225, t1071, t3046, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1348::<F>(t11132, t2435, t907);
        let t11136 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1349::<F>(t2854, t689);
        let t11138 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1350::<F>(t2859, t689);
    (t11075, t11084, t11108, t11119, t11120, t11121, t11128, t11132, t11133, t11134, t11136, t11138)
}
