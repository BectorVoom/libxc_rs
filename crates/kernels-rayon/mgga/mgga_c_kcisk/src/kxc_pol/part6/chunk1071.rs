//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1071/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1071(t14736: f64, t14743: f64, t1542: f64, t21748: f64, t2293: f64, t27516: f64, t30557: f64, t30561: f64, t30641: f64, t30644: f64, t30656: f64, t30660: f64, t30662: f64, t30664: f64, t30668: f64, t30731: f64, t31581: f64, t31584: f64, t31587: f64, t31603: f64, t516: f64, t6549: f64, t8378: f64, t8381: f64) -> f64 {
    let t31606 = t30731 - t30662 - t30664 + t30668 + t30557 - t30561 - t30641 - t30644 - t30660 + 0.17544670192365612213e1_f64 * t6549 * t8378 + 0.51947267698127589899e2_f64 * t21748 * t8381 - 0.1038945353962551798e3_f64 * t14736 * t31581 + 0.58482233974552040708e0_f64 * t1542 * t31584 + 0.1025389702100779493e4_f64 * t14743 * t31587 + 3.0_f64 * t27516 * t2293 - 0.19751789702565206229e-1_f64 * t30656 - 0.3109e-1_f64 * t31603 * t516;
    t31606
}
