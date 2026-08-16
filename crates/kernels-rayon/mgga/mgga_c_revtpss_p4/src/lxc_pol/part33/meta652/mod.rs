//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta652(t104480: f64, t1243: f64, t2149: f64, t1811: f64, t7642: f64, t8945: f64, t3596: f64, t13181: f64, t7635: f64, t1209: f64, t26948: f64, t29135: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t105354, t105365, t105383, t105404, t105409, t105420) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2103(t104480, t1243, t2149, t1811, t7642, t8945, t3596, t13181, t7635, t1209, t26948, t29135);
    (t105354, t105365, t105383, t105404, t105409, t105420)
}
