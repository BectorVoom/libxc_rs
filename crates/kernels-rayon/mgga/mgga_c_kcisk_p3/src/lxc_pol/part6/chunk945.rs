//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 945/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk945(t1974: f64, t29636: f64, t12098: f64, t12105: f64, t18558: f64, t18640: f64, t18643: f64, t1979: f64, t24727: f64, t2609: f64, t28456: f64, t28467: f64, t28470: f64, t29689: f64, t29692: f64, t29695: f64, t29700: f64, t29709: f64, t29712: f64, t29715: f64, t29718: f64, t5373: f64, t5398: f64, t5408: f64, t5415: f64, t7498: f64, t9109: f64, t9134: f64, t9137: f64, t9140: f64) -> f64 {
    let t29724 = t29636 * t1974;
    let t29727 = 0.96494049533612093922e2_f64 * t5398 * t29689 - 0.35089340384731224426e1_f64 * t5408 * t29692 + 0.51947267698127589897e2_f64 * t5415 * t29695 - 0.35089340384731224426e1_f64 * t18640 * t9134 + 0.35089340384731224426e1_f64 * t5415 * t29700 + 0.17544670192365612213e1_f64 * t24727 * t2609 + 0.17544670192365612213e1_f64 * t7498 * t9137 + 0.51947267698127589899e2_f64 * t18558 * t9140 - 0.1038945353962551798e3_f64 * t12098 * t29709 + 0.58482233974552040708e0_f64 * t1979 * t29712 + 0.1025389702100779493e4_f64 * t12105 * t29715 - 6.0_f64 * t5373 * t29718 - 0.19751789702565206229e-1_f64 * t28456 + t28467 - t28470 - 6.0_f64 * t18643 * t9109 + 6.0_f64 * t5398 * t29724;
    t29727
}
