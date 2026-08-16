//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 761/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk761(t4210: f64, t606: f64, t4186: f64, t60: f64, t1474: f64, t1480: f64, t2290: f64, t4202: f64, t4205: f64, t44: f64, t56: f64, t614: f64, t620: f64) -> (f64, f64, f64) {
    let t4211 = t4210 * t606;
    let t4214 = t60 * t4186;
    let t4217 = -20.0_f64 / 9.0_f64 * t614 * t1474 + 5.0_f64 / 18.0_f64 * t44 * t4202 + 5.0_f64 / 6.0_f64 * t44 * t4205 + 20.0_f64 / 9.0_f64 * t1480 * t620 + 5.0_f64 / 18.0_f64 * t56 * t4211 - 5.0_f64 / 6.0_f64 * t56 * t4214 - t2290;
    (t4211, t4214, t4217)
}
