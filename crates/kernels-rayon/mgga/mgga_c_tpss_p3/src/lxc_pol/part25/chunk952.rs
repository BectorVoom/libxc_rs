//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 952/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk952(t1125: f64, t12359: f64, t1501: f64, t242: f64, t9666: f64, t3062: f64, t4258: f64, t1113: f64, t1561: f64, t1014: f64, t450: f64, t1557: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12361 = t1125 * t12359 / 3456.0_f64;
    let t12367 = t242 * t9666 * t1501;
    let t12368 = t1125 * t12367;
    let t12371 = t4258 * t3062 / 432.0_f64;
    let t12377 = t1561 * t1113;
    let t12378 = t450 * t1014;
    let t12384 = t672 * t1557;
    (t12361, t12368, t12371, t12377, t12378, t12384)
}
