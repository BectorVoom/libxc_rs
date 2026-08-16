//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1121/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1121(t12378: f64, t581: f64, t12377: f64, t3068: f64, t1557: f64, t672: f64, t1098: f64, t1561: f64, t3054: f64, t1014: f64, t1113: f64, t12372: f64, t4046: f64) -> (f64, f64, f64, f64) {
    let t12379 = t12378 * t581;
    let t12380 = t12377 * t12379;
    let t12381 = t3068 * t12380;
    let t12384 = t672 * t1557;
    let t12385 = t1098 * t12384;
    let t12387 = t1561 * t3054;
    let t12389 = t1113 * t1014 * t581;
    let t12390 = t12387 * t12389;
    let t12391 = t3068 * t12390;
    let t12394 = t4046 * t12372;
    (t12381, t12385, t12391, t12394)
}
