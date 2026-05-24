//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 762/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk762<F: Float>(t12535: F, t12613: F, t1070: F, t3327: F, t242: F, t250: F, t3330: F, t15579: F, t1072: F, t1085: F, t12517: F, t12534: F, t12564: F, t12581: F, t12584: F, t12588: F, t12620: F, t15563: F, t15564: F, t15567: F, t15570: F, t15578: F, t15580: F, t15592: F, t15595: F, t15598: F, t251: F, t3329: F, t3349: F, t5: F, t56: F, t969: F) -> F {
    let t15599 = t12535 * t12613;
    let t15603 = F::new(1.0) / t3327 / t1070;
    let t15604 = t242 * t15603;
    let t15606 = F::new(1.0) / t3330 / t250;
    let t15607 = t15579 * t15606;
    let t15610 = F::cast_from(0.16562449037037037036e-2_f64) * t5 * t969 * t251 - t12588 - F::cast_from(0.1038945353962551798e3_f64) * t15563 * t15564 + F::cast_from(0.58482233974552040708e0_f64) * t1085 * t15567 + t12581 - t12584 + F::cast_from(0.35089340384731224426e1_f64) * t3349 * t15570 + F::cast_from(0.56969282336565386482e-3_f64) * t5 * t969 * t56 - t12564 - F::cast_from(0.19298809906722418785e3_f64) * t15578 * t15580 + F::new(1.0) * t1072 * t15592 + F::new(6.0) * t3329 * t15595 + F::cast_from(0.1025389702100779493e4_f64) * t15598 * t15599 + F::cast_from(0.20691336878655965246e4_f64) * t15604 * t15607 - t12534 + t12517 - t12620;
    t15610
}
