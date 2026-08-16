//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta278(t18742: f64, t2782: f64, t18681: f64, t231: f64, t2783: f64, t18677: f64, t2723: f64, t4503: f64, t6041: f64, t72: f64, t686: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18743, t18746, t18747, t18750, t18751, t18761, t18763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1052(t18742, t2782, t18681, t231, t2783, t18677, t2723, t4503, t6041, t72, t686, t874);
    (t18743, t18746, t18747, t18750, t18751, t18761, t18763)
}
