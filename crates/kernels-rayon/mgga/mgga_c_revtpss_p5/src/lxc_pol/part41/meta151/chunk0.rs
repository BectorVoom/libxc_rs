//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 678/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk678(t1448: f64, t1450: f64, t565: f64, t2219: f64, t2223: f64, t2226: f64, t2230: f64, t2233: f64, t2239: f64, t1466: f64, t602: f64, t1497: f64, t644: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4140 = t1448 * t1450;
    let t4146 = t565 * t565;
    let t4147 = 1.0_f64 / t4146;
    let t4171 = -t2219 + t2223 - t2226 + t2230 - t2233 + t2239;
    let t4173 = t1466 * t602;
    let t4178 = t1497 * t644;
    (t4140, t4146, t4147, t4171, t4173, t4178)
}
