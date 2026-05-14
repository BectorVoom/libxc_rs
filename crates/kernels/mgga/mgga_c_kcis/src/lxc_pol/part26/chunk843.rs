//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 843/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk843<F: Float>(t21333: F, t5619: F, t1578: F, t7024: F, t45: F, t6996: F, t1345: F, t1357: F, t21172: F, t21174: F, t21176: F, t21178: F, t21180: F, t21293: F, t21295: F, t21311: F, t21316: F, t21320: F, t21324: F, t21327: F, t21330: F, t5590: F, t5615: F) -> (F, F, F) {
    let t21334 = t21333 * t5619;
    let t21337 = t7024 * t1578;
    let t21342 = t45 * t6996;
    let t21345 = t21172 + t21174 + t21176 - t21178 + t21180 + t21293 + t21295 + 0.19751789702565206229e-1 * t45 * t21311 - 0.1025389702100779493e4 * t1345 * t21316 + 0.1038945353962551798e3 * t1345 * t21320 + 0.11696446794910408142e1 * t1345 * t21324 + 0.23392893589820816284e1 * t1345 * t21327 - 0.34631511798751726598e2 * t1345 * t21330 - 0.17315755899375863299e2 * t1345 * t21334 - 0.35089340384731224426e1 * t1345 * t21337 - 0.11696446794910408142e1 * t5590 * t5615 - 0.58482233974552040708e0 * t21342 * t1357;
    (t21334, t21337, t21345)
}
