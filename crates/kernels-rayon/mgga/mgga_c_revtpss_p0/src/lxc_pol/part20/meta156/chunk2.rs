//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 849/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk849(t3: f64, t4153: f64, t116: f64, t2327: f64, t117: f64, t2371: f64, t1459: f64, t1461: f64, t572: f64, t573: f64, t670: f64, t94: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4154 = t3 * t4153;
    let t4158 = param_d * t4153;
    let t4162 = t116 * t2327;
    let t4165 = t117 * t2371;
    let t4168 = 6.0_f64 * t1459 * t1461 + t4158 * t573 + 6.0_f64 * t4162 * t572 + 3.0_f64 * t4165 * t572;
    let t4254 = t94 * t670;
    (t4154, t4158, t4162, t4165, t4168, t4254)
}
