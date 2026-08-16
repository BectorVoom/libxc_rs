//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta284(t4067: f64, t786: f64, t1364: f64, t213: f64, t4066: f64, t1420: f64, t1426: f64, t3917: f64, t64: f64, t843: f64) -> (f64, f64, f64, f64, f64) {
        let (t10169, t10171, t10175, t10176, t10199) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1033(t4067, t786, t1364, t213, t4066, t1420, t1426, t3917, t64, t843);
    (t10169, t10171, t10175, t10176, t10199)
}
