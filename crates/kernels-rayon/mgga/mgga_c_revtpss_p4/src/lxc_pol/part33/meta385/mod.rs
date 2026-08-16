//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta385(t1011: f64, t16219: f64, t15688: f64, t3299: f64, t1678: f64, t3057: f64, t4930: f64, t994: f64, t3046: f64, t379: f64, t1078: f64, t1651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16220, t16226, t16284, t16302, t16305, t16312, t16313) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1432(t1011, t16219, t15688, t3299, t1678, t3057, t4930, t994, t3046, t379, t1078, t1651);
    (t16220, t16226, t16284, t16302, t16305, t16312, t16313)
}
