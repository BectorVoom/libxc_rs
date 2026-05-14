//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1132/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1132<F: Float>(t1347: F, t1356: F, t16330: F, t4367: F, t5618: F, t3918: F, t5613: F, t1578: F, t1345: F, t16251: F, t16253: F, t16256: F, t16259: F, t16262: F, t16266: F, t16269: F, t16273: F, t16276: F, t16277: F, t16281: F, t3914: F, t5596: F, t5615: F, t5620: F) -> (F, F, F, F) {
    let t16332 = t1347 * t16330 * t1356;
    let t16337 = t5618 * t4367;
    let t16344 = t3918 * t5613;
    let t16345 = t16344 * t1578;
    let t16348 = -t16251 + t16253 - t16256 - t16259 - t16262 + t16266 + t16269 + t16273 + t16276 - 0.17315755899375863299e2 * t1345 * t16277 + 0.1038945353962551798e3 * t1345 * t16281 - 0.58482233974552040708e0 * t1345 * t16332 - 0.11696446794910408142e1 * t3914 * t5615 - 0.35089340384731224426e1 * t1345 * t16337 - 0.34631511798751726598e2 * t3914 * t5620 + 0.23392893589820816284e1 * t3914 * t5596 + 0.23392893589820816284e1 * t1345 * t16345;
    (t16332, t16337, t16345, t16348)
}
