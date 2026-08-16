//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 945/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk945<F: Float>(t1974: F, t29636: F, t12098: F, t12105: F, t18558: F, t18640: F, t18643: F, t1979: F, t24727: F, t2609: F, t28456: F, t28467: F, t28470: F, t29689: F, t29692: F, t29695: F, t29700: F, t29709: F, t29712: F, t29715: F, t29718: F, t5373: F, t5398: F, t5408: F, t5415: F, t7498: F, t9109: F, t9134: F, t9137: F, t9140: F) -> F {
    let t29724 = t29636 * t1974;
    let t29727 = F::cast_from(0.96494049533612093922e2_f64) * t5398 * t29689 - F::cast_from(0.35089340384731224426e1_f64) * t5408 * t29692 + F::cast_from(0.51947267698127589897e2_f64) * t5415 * t29695 - F::cast_from(0.35089340384731224426e1_f64) * t18640 * t9134 + F::cast_from(0.35089340384731224426e1_f64) * t5415 * t29700 + F::cast_from(0.17544670192365612213e1_f64) * t24727 * t2609 + F::cast_from(0.17544670192365612213e1_f64) * t7498 * t9137 + F::cast_from(0.51947267698127589899e2_f64) * t18558 * t9140 - F::cast_from(0.1038945353962551798e3_f64) * t12098 * t29709 + F::cast_from(0.58482233974552040708e0_f64) * t1979 * t29712 + F::cast_from(0.1025389702100779493e4_f64) * t12105 * t29715 - F::cast_from(6.0_f64) * t5373 * t29718 - F::cast_from(0.19751789702565206229e-1_f64) * t28456 + t28467 - t28470 - F::cast_from(6.0_f64) * t18643 * t9109 + F::cast_from(6.0_f64) * t5398 * t29724;
    t29727
}
