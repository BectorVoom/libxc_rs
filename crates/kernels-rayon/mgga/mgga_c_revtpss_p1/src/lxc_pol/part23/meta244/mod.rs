//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta244(t1340: f64, t9387: f64, t1320: f64, t3853: f64, t123: f64, t147: f64, t9291: f64) -> (f64, f64, f64) {
        let (t9389, t9391, t9394) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1420(t1340, t9387, t1320, t3853, t123, t147, t9291);
    (t9389, t9391, t9394)
}
