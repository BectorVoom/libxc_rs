//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1119/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1119(t12333: f64, t12351: f64, t450: f64, t1112: f64, t242: f64, t3090: f64, t4056: f64, t1125: f64, t1128: f64, t11846: f64, t1501: f64, t9666: f64) -> (f64, f64, f64, f64, f64) {
    let t12352 = t12333 + t12351;
    let t12353 = t12352 * t450;
    let t12355 = t242 * t1112 * t12353;
    let t12359 = t242 * t3090 * t4056;
    let t12361 = t1125 * t12359 / 3456.0_f64;
    let t12363 = t242 * t1128 * t11846;
    let t12367 = t242 * t9666 * t1501;
    (t12352, t12355, t12361, t12363, t12367)
}
