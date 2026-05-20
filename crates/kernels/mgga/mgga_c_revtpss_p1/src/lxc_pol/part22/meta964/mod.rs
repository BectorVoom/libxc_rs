//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta964 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3226;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta964<F: Float>(t14383: F, t4311: F, t40092: F, t40094: F, t50047: F, t14386: F, t4305: F, t1544: F, t2832: F, t157: F, t2251: F, t6002: F, t15071: F, t1940: F, t2403: F, t40084: F, t40088: F, t40099: F, t40103: F, t40115: F, t4556: F) -> (F, F, F, F, F, F, F) {
        let (t61197, t61198, t61199, t61200, t61202, t61203, t61209) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3226::<F>(t14383, t4311, t40092, t40094, t50047, t14386, t4305, t1544, t2832, t157, t2251, t6002);
        let t61210 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3227::<F>(t15071, t1940, t2403, t40084, t40088, t40099, t40103, t40115, t4556, t61197, t61198, t61199, t61200, t61202, t61203, t61209);
    (t61197, t61198, t61199, t61200, t61202, t61209, t61210)
}
