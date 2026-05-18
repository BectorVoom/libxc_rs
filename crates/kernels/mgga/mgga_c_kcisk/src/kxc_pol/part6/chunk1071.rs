//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1071/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1071<F: Float>(t14736: F, t14743: F, t1542: F, t21748: F, t2293: F, t27516: F, t30557: F, t30561: F, t30641: F, t30644: F, t30656: F, t30660: F, t30662: F, t30664: F, t30668: F, t30731: F, t31581: F, t31584: F, t31587: F, t31603: F, t516: F, t6549: F, t8378: F, t8381: F) -> F {
    let t31606 = t30731 - t30662 - t30664 + t30668 + t30557 - t30561 - t30641 - t30644 - t30660 + F::new(0.17544670192365612213e1) * t6549 * t8378 + F::new(0.51947267698127589899e2) * t21748 * t8381 - F::new(0.1038945353962551798e3) * t14736 * t31581 + F::new(0.58482233974552040708e0) * t1542 * t31584 + F::new(0.1025389702100779493e4) * t14743 * t31587 + F::new(3.0) * t27516 * t2293 - F::new(0.19751789702565206229e-1) * t30656 - F::new(0.3109e-1) * t31603 * t516;
    t31606
}
