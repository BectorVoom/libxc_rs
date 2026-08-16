//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1855/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1855(t19049: f64, t983: f64, t15547: f64, t1642: f64, t4719: f64, t4725: f64, t6104: f64, t914: f64, t936: f64, t15416: f64, t1610: f64, t4590: f64, t4632: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19051 = 0.5848223622634646207e0_f64 * t19049 * t983;
    let t19053 = 0.11696447245269292414e1_f64 * t15547 * t1642;
    let t19055 = 0.23392894490538584828e1_f64 * t4719 * t4725;
    let t19056 = t6104 * t914;
    let t19058 = 1.0_f64 * t19056 * t936;
    let t19060 = 2.0_f64 * t15416 * t1610;
    let t19062 = 2.0_f64 * t4590 * t4632;
    (t19051, t19053, t19055, t19056, t19058, t19060, t19062)
}
