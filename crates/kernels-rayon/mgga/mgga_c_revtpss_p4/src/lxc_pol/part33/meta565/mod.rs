//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1965;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta565(t1287: f64, t30763: f64, t2142: f64, t6702: f64, t26969: f64, t6744: f64, t7652: f64, t2138: f64, t6601: f64, t343: f64, t5842: f64, t136: f64, t1797: f64, t1808: f64, t26821: f64, t26844: f64, t26849: f64, t26867: f64, t26880: f64, t29020: f64, t29023: f64, t29027: f64, t29031: f64, t29034: f64, t29037: f64, t29065: f64, t29083: f64, t464: f64, t484: f64, t6619: f64, t6625: f64, t6631: f64, t6635: f64, t6640: f64, t6679: f64, t7618: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30764, t30767) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1965(t1287, t30763, t2142, t6702);
        let (t30768, t30771, t30772, t30789, t30799, t30800, t30805) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1966(t26969, t30767, t2142, t6744, t7652, t2138, t6601, t343, t5842, t136, t1797, t1808, t26821, t26844, t26849, t26867, t26880, t29020, t29023, t29027, t29031, t29034, t29037, t29065, t29083, t464, t484, t6619, t6625, t6631, t6635, t6640, t6679, t7618, t7624);
    (t30764, t30767, t30768, t30771, t30772, t30789, t30799, t30800, t30805)
}
