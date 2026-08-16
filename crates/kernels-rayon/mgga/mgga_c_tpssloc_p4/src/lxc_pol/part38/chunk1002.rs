//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1002/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1002(t1294: f64, t9722: f64, t172: f64, t3681: f64, t763: f64, t2528: f64, t3691: f64, t9919: f64, t2663: f64, t3814: f64, t67: f64, t758: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12087 = 0.10389515463408878255e3_f64 * t1294 * t9722;
    let t12088 = t3681 * t172;
    let t12089 = t12088 * t763;
    let t12091 = t3691 * t2528;
    let t12094 = 0.35089341735807877242e1_f64 * t1294 * t9919;
    let t12097 = t3814 * t2663;
    let t12099 = t3681 * t67;
    let t12100 = t12099 * t758;
    (t12087, t12089, t12091, t12094, t12097, t12100)
}
