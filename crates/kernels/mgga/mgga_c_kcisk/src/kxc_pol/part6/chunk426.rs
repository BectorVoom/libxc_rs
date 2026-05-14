//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 426/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk426<F: Float>(t240: F, t5: F, t1091: F, t1094: F, t2849: F, t2853: F, t2860: F, t2882: F, t2890: F, t2892: F, t2896: F, t2903: F, t2914: F, t2922: F, t3353: F, t1097: F, t1100: F) -> (F, F, F) {
    let t3357 = t240 * t5;
    let t3366 = -t2849 - t2853 - t2860 + t2882 + t2890 + t240 * t3353 + 0.24415406715670879921e-3 * t1091 * t2892 + 0.10843580882781524214e-1 * t3357 * t2896 + 0.11696446794910408142e1 * t1094 * t2903 - 0.58482233974552040708e0 * t1094 * t2914 - 0.17315755899375863299e2 * t1094 * t2922;
    let t3368 = t1097 * t1100;
    (t3357, t3366, t3368)
}
