//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 762/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk762(t12535: f64, t12613: f64, t1070: f64, t3327: f64, t242: f64, t250: f64, t3330: f64, t15579: f64, t1072: f64, t1085: f64, t12517: f64, t12534: f64, t12564: f64, t12581: f64, t12584: f64, t12588: f64, t12620: f64, t15563: f64, t15564: f64, t15567: f64, t15570: f64, t15578: f64, t15580: f64, t15592: f64, t15595: f64, t15598: f64, t251: f64, t3329: f64, t3349: f64, t5: f64, t56: f64, t969: f64) -> f64 {
    let t15599 = t12535 * t12613;
    let t15603 = 1.0_f64 / t3327 / t1070;
    let t15604 = t242 * t15603;
    let t15606 = 1.0_f64 / t3330 / t250;
    let t15607 = t15579 * t15606;
    let t15610 = 0.16562449037037037036e-2_f64 * t5 * t969 * t251 - t12588 - 0.1038945353962551798e3_f64 * t15563 * t15564 + 0.58482233974552040708e0_f64 * t1085 * t15567 + t12581 - t12584 + 0.35089340384731224426e1_f64 * t3349 * t15570 + 0.56969282336565386482e-3_f64 * t5 * t969 * t56 - t12564 - 0.19298809906722418785e3_f64 * t15578 * t15580 + 1.0_f64 * t1072 * t15592 + 6.0_f64 * t3329 * t15595 + 0.1025389702100779493e4_f64 * t15598 * t15599 + 0.20691336878655965246e4_f64 * t15604 * t15607 - t12534 + t12517 - t12620;
    t15610
}
