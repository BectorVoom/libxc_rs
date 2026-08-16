//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1245/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1245(t19045: f64, t324: f64, t300: f64, t6184: f64, t983: f64, t15547: f64, t1642: f64, t4719: f64, t4725: f64, t6104: f64, t914: f64, t936: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19046 = t19045 * t324;
    let t19048 = 0.19751673498613801407e-1_f64 * t300 * t19046;
    let t19049 = t300 * t6184;
    let t19051 = 0.5848223622634646207e0_f64 * t19049 * t983;
    let t19053 = 0.11696447245269292414e1_f64 * t15547 * t1642;
    let t19055 = 0.23392894490538584828e1_f64 * t4719 * t4725;
    let t19056 = t6104 * t914;
    let t19058 = 1.0_f64 * t19056 * t936;
    (t19046, t19048, t19051, t19053, t19055, t19058)
}
