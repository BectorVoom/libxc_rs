//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta298(t2516: f64, t676: f64, t3869: f64, t2496: f64, t1386: f64, t2681: f64, t820: f64, t1401: f64, t4000: f64, t843: f64, t136: f64, t4011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9863, t9865, t9866, t9868, t9909, t9910, t9918, t9921) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1289(t2516, t676, t3869, t2496, t1386, t2681, t820, t1401, t4000, t843, t136, t4011);
    (t9863, t9865, t9866, t9868, t9909, t9910, t9918, t9921)
}
