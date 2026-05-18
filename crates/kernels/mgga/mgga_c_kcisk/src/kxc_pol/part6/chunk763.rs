//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 763/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk763<F: Float>(t1094: F, t12517: F, t12534: F, t12564: F, t12568: F, t12573: F, t12581: F, t12584: F, t12589: F, t12592: F, t12595: F, t15559: F, t15610: F, t240: F, t3357: F) -> F {
    let t15613 = F::new(0.1038945353962551798e3) * t1094 * t12568 - t12517 + t12534 + t12564 - F::new(0.21687161765563048428e-1) * t3357 * t12589 + F::new(0.16265371324172286321e-1) * t3357 * t12592 + F::new(0.48159446095139119799e0) * t3357 * t12595 - F::new(0.51947267698127589897e2) * t1094 * t12573 + t240 * (t15559 + t15610) - t12581 + t12584;
    t15613
}
