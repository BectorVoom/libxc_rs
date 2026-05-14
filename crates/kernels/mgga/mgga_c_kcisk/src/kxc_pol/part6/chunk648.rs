//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 648/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk648<F: Float>(t32: F, t5: F, t969: F, t12517: F, t12534: F, t12537: F, t12541: F, t12554: F, t12564: F, t12568: F, t12573: F, t12576: F, t12581: F, t2895: F, t839: F, t12514: F, t830: F) -> (F, F, F) {
    let t12584 = 0.34451131037037037036e-2 * t5 * t969 * t32;
    let t12585 = -t12517 + t12534 - 0.35089340384731224426e1 * t839 * t12537 + 0.35089340384731224426e1 * t839 * t12541 - 0.58482233974552040708e0 * t839 * t12554 + t12564 + 0.1038945353962551798e3 * t839 * t12568 - 0.51947267698127589897e2 * t839 * t12573 - 0.32530742648344572643e-1 * t2895 * t12576 - t12581 + t12584;
    let t12586 = t12514 * t830;
    (t12584, t12585, t12586)
}
