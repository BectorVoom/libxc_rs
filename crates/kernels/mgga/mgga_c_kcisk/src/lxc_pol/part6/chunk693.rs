//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 693/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk693<F: Float>(t1078: F, t15591: F, t15579: F, t12610: F, t47: F, t12535: F, t12613: F, t1070: F, t3327: F, t242: F, t250: F, t3330: F, t1072: F, t1085: F, t12517: F, t12534: F, t12564: F, t12581: F, t12584: F, t12588: F, t12620: F, t15563: F, t15564: F, t15567: F, t15570: F, t15578: F, t15580: F, t251: F, t3329: F, t3349: F, t5: F, t56: F, t969: F) -> (F,) {
    let t15592 = t15591 * t1078;
    let t15595 = t15579 * t1078;
    let t15598 = t47 * t12610;
    let t15599 = t12535 * t12613;
    let t15603 = 1.0 / t3327 / t1070;
    let t15604 = t242 * t15603;
    let t15606 = 1.0 / t3330 / t250;
    let t15607 = t15579 * t15606;
    let t15610 = 0.16562449037037037036e-2 * t5 * t969 * t251 - t12588 - 0.1038945353962551798e3 * t15563 * t15564 + 0.58482233974552040708e0 * t1085 * t15567 + t12581 - t12584 + 0.35089340384731224426e1 * t3349 * t15570 + 0.56969282336565386482e-3 * t5 * t969 * t56 - t12564 - 0.19298809906722418785e3 * t15578 * t15580 + 1.0 * t1072 * t15592 + 6.0 * t3329 * t15595 + 0.1025389702100779493e4 * t15598 * t15599 + 0.20691336878655965246e4 * t15604 * t15607 - t12534 + t12517 - t12620;
    (t15610,)
}
