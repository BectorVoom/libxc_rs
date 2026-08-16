//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1965;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta565<F: Float>(t1287: F, t30763: F, t2142: F, t6702: F, t26969: F, t6744: F, t7652: F, t2138: F, t6601: F, t343: F, t5842: F, t136: F, t1797: F, t1808: F, t26821: F, t26844: F, t26849: F, t26867: F, t26880: F, t29020: F, t29023: F, t29027: F, t29031: F, t29034: F, t29037: F, t29065: F, t29083: F, t464: F, t484: F, t6619: F, t6625: F, t6631: F, t6635: F, t6640: F, t6679: F, t7618: F, t7624: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t30764, t30767) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1965::<F>(t1287, t30763, t2142, t6702);
        let (t30768, t30771, t30772, t30789, t30799, t30800, t30805) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1966::<F>(t26969, t30767, t2142, t6744, t7652, t2138, t6601, t343, t5842, t136, t1797, t1808, t26821, t26844, t26849, t26867, t26880, t29020, t29023, t29027, t29031, t29034, t29037, t29065, t29083, t464, t484, t6619, t6625, t6631, t6635, t6640, t6679, t7618, t7624);
    (t30764, t30767, t30768, t30771, t30772, t30789, t30799, t30800, t30805)
}
