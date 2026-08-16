//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 525/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk525(t15: f64, t2545: f64, t60: f64, t762: f64, t647: f64, t130: f64, t20: f64, t21: f64, t736: f64, t97: f64, t787: f64, t5: f64, t728: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2546 = t2545 * t15;
    let t2551 = t60 * t762;
    let t2552 = t2551 * t647;
    let t2553 = t130 * t20;
    let t2555 = t2553 * t21 * t736;
    let t2558 = t15 * t97;
    let t2559 = t787 * t2558;
    let t2561 = t5 * t88 * t728;
    (t2546, t2551, t2552, t2553, t2555, t2558, t2559, t2561)
}
