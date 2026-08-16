//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1053/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1053(t12070: f64, t12081: f64, t157: f64, t182: f64, t1294: f64, t9722: f64, t172: f64, t3681: f64, t763: f64, t2528: f64, t3691: f64, t9919: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12083 = (t12070 + t12081) * t157;
    let t12085 = 0.19751673498613801407e-1_f64 * t12083 * t182;
    let t12087 = 0.10389515463408878255e3_f64 * t1294 * t9722;
    let t12088 = t3681 * t172;
    let t12089 = t12088 * t763;
    let t12090 = 0.17544670867903938621e1_f64 * t12089;
    let t12091 = t3691 * t2528;
    let t12092 = 0.51947577317044391276e2_f64 * t12091;
    let t12094 = 0.35089341735807877242e1_f64 * t1294 * t9919;
    (t12083, t12085, t12087, t12090, t12092, t12094)
}
