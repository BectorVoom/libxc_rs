//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 694/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk694<F: Float>(t1094: F, t12517: F, t12534: F, t12564: F, t12568: F, t12573: F, t12581: F, t12584: F, t12589: F, t12592: F, t12595: F, t15559: F, t15610: F, t240: F, t3357: F, t1091: F, t12537: F, t12541: F, t12554: F, t12576: F, t12588: F, t12601: F, t12604: F, t12608: F, t12614: F, t12620: F, t12624: F, t12626: F) -> (F, F) {
    let t15613 = 0.1038945353962551798e3 * t1094 * t12568 - t12517 + t12534 + t12564 - 0.21687161765563048428e-1 * t3357 * t12589 + 0.16265371324172286321e-1 * t3357 * t12592 + 0.48159446095139119799e0 * t3357 * t12595 - 0.51947267698127589897e2 * t1094 * t12573 + t240 * (t15559 + t15610) - t12581 + t12584;
    let t15626 = -0.1025389702100779493e4 * t1094 * t12614 + t12588 - 0.32530742648344572643e-1 * t3357 * t12576 - 0.56969282336565386482e-3 * t1091 * t12626 + t12601 - t12604 - 0.35089340384731224426e1 * t1094 * t12537 + 0.35089340384731224426e1 * t1094 * t12541 - t12608 - 0.58482233974552040708e0 * t1094 * t12554 + t12620 + t12624;
    (t15613, t15626)
}
