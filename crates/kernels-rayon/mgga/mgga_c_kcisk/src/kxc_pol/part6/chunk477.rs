//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 477/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk477(t398: f64, t4374: f64, t3532: f64, t539: f64, t1588: f64, t442: f64, t1390: f64, t397: f64, t3979: f64, t535: f64, t24: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4375 = t398 * t4374;
    let t4391 = t539 * t3532;
    let t4400 = t1588 * t442;
    let t4406 = t539 * t1390;
    let t4416 = t397 * t3979 * t539;
    let t4418 = 0.59969295720591057378e-2_f64 * t535 * t4416;
    let t4419 = t397 * t24;
    (t4375, t4391, t4400, t4406, t4416, t4418, t4419)
}
