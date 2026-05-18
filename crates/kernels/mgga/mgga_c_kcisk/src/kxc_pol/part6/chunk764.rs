//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 764/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk764<F: Float>(t1091: F, t1094: F, t12537: F, t12541: F, t12554: F, t12576: F, t12588: F, t12601: F, t12604: F, t12608: F, t12614: F, t12620: F, t12624: F, t12626: F, t3357: F) -> F {
    let t15626 = -F::new(0.1025389702100779493e4) * t1094 * t12614 + t12588 - F::new(0.32530742648344572643e-1) * t3357 * t12576 - F::new(0.56969282336565386482e-3) * t1091 * t12626 + t12601 - t12604 - F::new(0.35089340384731224426e1) * t1094 * t12537 + F::new(0.35089340384731224426e1) * t1094 * t12541 - t12608 - F::new(0.58482233974552040708e0) * t1094 * t12554 + t12620 + t12624;
    t15626
}
