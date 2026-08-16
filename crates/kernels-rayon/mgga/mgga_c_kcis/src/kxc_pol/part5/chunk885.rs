//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 885/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk885(t552: f64, t7380: f64, t573: f64, t41: f64, t7052: f64, t556: f64, t571: f64, t2042: f64, t2046: f64, t6927: f64, t572: f64, t4255: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7381 = t7380 * t552;
    let t7382 = t7381 * sigma2;
    let t7383 = t7382 * t573;
    let t7385 = t7052 * t41;
    let t7386 = t7385 * t556;
    let t7387 = t571 * t7386;
    let t7389 = t2042 * t2046;
    let t7390 = t571 * t7389;
    let t7392 = t556 * t6927;
    let t7393 = t572 * t7392;
    let t7394 = t4255 * t7393;
    (t7382, t7383, t7385, t7386, t7387, t7389, t7390, t7392, t7393, t7394)
}
