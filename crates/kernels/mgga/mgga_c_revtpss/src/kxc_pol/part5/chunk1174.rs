//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1174/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1174<F: Float>(t1134: F, t20361: F, t12331: F, t6442: F, t5079: F, t5087: F, t3407: F, t6449: F, t1139: F, t20337: F, t12254: F, t20293: F, t141: F, t12542: F, t12543: F, t16710: F, t16931: F, t17131: F, t17140: F) -> (F, F, F, F, F, F, F) {
    let t20362 = t20361 * t1134;
    let t20365 = t12331 * t6442;
    let t20366 = t20365 * t1134;
    let t20368 = t5087 * t5079;
    let t20370 = t3407 * t6449;
    let t20371 = t20370 * t1134;
    let t20373 = t1139 * t20337;
    let t20377 = t12254 * t20293;
    let t20378 = t141 * t20377;
    let t20380 = -0.412621875e-1 * t20366 + 0.16504875e0 * t20368 + 0.82524375e-1 * t20371 - t17131 - t12542 - t12543 + 0.16504875e0 * t20373 - 0.40256666666666666668e0 * t16710 + t17140 + 0.36793333333333333333e-1 * t16931 + 0.36793333333333333333e-1 * t20378;
    (t20362, t20366, t20368, t20371, t20373, t20378, t20380)
}
