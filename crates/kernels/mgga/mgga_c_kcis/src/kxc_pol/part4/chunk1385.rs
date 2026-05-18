//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1385/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1385<F: Float>(t187: F, t5586: F, t1357: F, t1585: F, t16100: F, t16108: F, t16112: F, t16259: F, t16262: F, t16266: F, t16269: F, t16273: F, t16276: F, t16277: F, t16281: F, t16332: F, t16337: F, t16345: F, t4381: F, t5596: F, t5615: F, t5620: F) -> F {
    let t17942 = t187 * t5586;
    let t17959 = -t16259 - t16262 + t16266 + t16269 + t16273 + t16276 + F::new(0.11696446794910408142e1) * t1585 * t16100 - F::new(0.34631511798751726598e2) * t1585 * t16112 - F::new(0.17315755899375863299e2) * t1585 * t16277 - F::new(0.11696446794910408142e1) * t4381 * t5615 - F::new(0.11696446794910408142e1) * t17942 * t1357 + F::new(0.23392893589820816284e1) * t4381 * t5596 - F::new(0.1025389702100779493e4) * t1585 * t16108 + F::new(0.23392893589820816284e1) * t1585 * t16345 - F::new(0.58482233974552040708e0) * t1585 * t16332 + F::new(0.1038945353962551798e3) * t1585 * t16281 - F::new(0.35089340384731224426e1) * t1585 * t16337 - F::new(0.34631511798751726598e2) * t4381 * t5620;
    t17959
}
