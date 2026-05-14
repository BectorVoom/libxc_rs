//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 844/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk844<F: Float>(t24774: F, t2604: F, t2609: F, t8607: F, t2394: F, t24747: F, t1685: F, t28341: F, t4790: F, t28507: F, t10699: F, t2605: F, t9124: F, t1974: F, t29636: F, t12098: F, t12105: F, t18558: F, t18640: F, t18643: F, t1979: F, t24727: F, t28456: F, t28467: F, t28470: F, t5373: F, t5398: F, t5408: F, t5415: F, t7498: F, t9109: F, t9134: F, t9137: F, t9140: F) -> (F,) {
    let t29689 = t24774 * t2604;
    let t29692 = t2609 * t8607;
    let t29695 = t24747 * t2394;
    let t29700 = t28341 * t1685;
    let t29709 = t28341 * t4790;
    let t29712 = t28507 * t1685;
    let t29715 = t28341 * t10699;
    let t29718 = t2605 * t9124;
    let t29724 = t29636 * t1974;
    let t29727 = 0.96494049533612093922e2 * t5398 * t29689 - 0.35089340384731224426e1 * t5408 * t29692 + 0.51947267698127589897e2 * t5415 * t29695 - 0.35089340384731224426e1 * t18640 * t9134 + 0.35089340384731224426e1 * t5415 * t29700 + 0.17544670192365612213e1 * t24727 * t2609 + 0.17544670192365612213e1 * t7498 * t9137 + 0.51947267698127589899e2 * t18558 * t9140 - 0.1038945353962551798e3 * t12098 * t29709 + 0.58482233974552040708e0 * t1979 * t29712 + 0.1025389702100779493e4 * t12105 * t29715 - 6.0 * t5373 * t29718 - 0.19751789702565206229e-1 * t28456 + t28467 - t28470 - 6.0 * t18643 * t9109 + 6.0 * t5398 * t29724;
    (t29727,)
}
