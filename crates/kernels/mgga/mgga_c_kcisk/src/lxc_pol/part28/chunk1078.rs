//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1078/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1078<F: Float>(t1974: F, t24721: F, t1676: F, t8584: F, t12098: F, t12114: F, t18553: F, t18558: F, t18640: F, t1966: F, t1980: F, t22786: F, t24661: F, t24668: F, t24671: F, t24674: F, t2609: F, t5408: F, t5415: F, t7498: F, t7503: F, t7506: F, t7510: F, t764: F, t9128: F) -> (F,) {
    let t24722 = t24721 * t1974;
    let t24727 = t8584 * t1676;
    let t24734 = -0.3109e-1 * t24661 * t764 - 0.23392893589820816284e1 * t18640 * t7503 + 0.346315117987517266e2 * t18558 * t7510 + 0.35089340384731224426e1 * t5415 * t24668 - 0.23392893589820816284e1 * t5408 * t24671 - 0.1038945353962551798e3 * t12098 * t24674 + t22786 + 1.0 * t1966 * t24722 + 0.32164683177870697974e2 * t12114 * t9128 + 0.58482233974552040708e0 * t24727 * t1980 + 0.11696446794910408142e1 * t18553 * t2609 + 0.11696446794910408142e1 * t7498 * t7506;
    (t24734,)
}
