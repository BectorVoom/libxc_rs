//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 974/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk974(t187: f64, t5586: f64, t1357: f64, t1585: f64, t16100: f64, t16108: f64, t16112: f64, t16259: f64, t16262: f64, t16266: f64, t16269: f64, t16273: f64, t16276: f64, t16277: f64, t16281: f64, t16332: f64, t16337: f64, t16345: f64, t4381: f64, t5596: f64, t5615: f64, t5620: f64) -> f64 {
    let t17942 = t187 * t5586;
    let t17959 = -t16259 - t16262 + t16266 + t16269 + t16273 + t16276 + 0.11696446794910408142e1_f64 * t1585 * t16100 - 0.34631511798751726598e2_f64 * t1585 * t16112 - 0.17315755899375863299e2_f64 * t1585 * t16277 - 0.11696446794910408142e1_f64 * t4381 * t5615 - 0.11696446794910408142e1_f64 * t17942 * t1357 + 0.23392893589820816284e1_f64 * t4381 * t5596 - 0.1025389702100779493e4_f64 * t1585 * t16108 + 0.23392893589820816284e1_f64 * t1585 * t16345 - 0.58482233974552040708e0_f64 * t1585 * t16332 + 0.1038945353962551798e3_f64 * t1585 * t16281 - 0.35089340384731224426e1_f64 * t1585 * t16337 - 0.34631511798751726598e2_f64 * t4381 * t5620;
    t17959
}
