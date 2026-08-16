//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1804;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta498<F: Float>(t25304: F, t7283: F, t25946: F, t25949: F, t786: F, t7286: F, t1426: F, t3999: F, t213: F, t7274: F, t116: F, t7002: F, t10301: F, t7565: F, t38: F, t7574: F, t2247: F, t2282: F, t55: F, t10309: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26069, t26071, t26072, t26073, t26079, t26084, t26123) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1804::<F>(t25304, t7283, t25946, t25949, t786, t7286, t1426, t3999, t213, t7274, t116, t7002);
        let (t26749, t26754, t26755, t26776, t26792) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1805::<F>(t10301, t7565, t38, t7574, t2247, t2282, t55, t10309);
    (t26069, t26071, t26072, t26073, t26079, t26084, t26123, t26749, t26754, t26755, t26776, t26792)
}
