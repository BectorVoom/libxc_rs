//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta198(t1121: f64, t1263: f64, t1214: f64, t1469: f64, t1042: f64, t3362: f64, t3617: f64, t4181: f64, t1012: f64, t1224: f64, t5052: f64, t3698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5296, t5297, t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk877(t1121, t1263, t1214, t1469, t1042, t3362, t3617, t4181, t1012, t1224, t5052, t3698);
    (t5296, t5297, t5298, t5299, t5302, t5303, t5304, t5308, t5309, t5312)
}
