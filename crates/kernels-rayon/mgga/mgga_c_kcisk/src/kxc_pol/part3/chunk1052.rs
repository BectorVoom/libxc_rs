//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1052/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1052(t1091: f64, t1094: f64, t12537: f64, t12541: f64, t12554: f64, t12576: f64, t12588: f64, t12601: f64, t12604: f64, t12608: f64, t12614: f64, t12620: f64, t12624: f64, t12626: f64, t3357: f64) -> f64 {
    let t15626 = -0.1025389702100779493e4_f64 * t1094 * t12614 + t12588 - 0.32530742648344572643e-1_f64 * t3357 * t12576 - 0.56969282336565386482e-3_f64 * t1091 * t12626 + t12601 - t12604 - 0.35089340384731224426e1_f64 * t1094 * t12537 + 0.35089340384731224426e1_f64 * t1094 * t12541 - t12608 - 0.58482233974552040708e0_f64 * t1094 * t12554 + t12620 + t12624;
    t15626
}
