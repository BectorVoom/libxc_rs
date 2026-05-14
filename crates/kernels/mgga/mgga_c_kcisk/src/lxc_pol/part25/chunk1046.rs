//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1046/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1046<F: Float>(t16309: F, t16313: F, t16346: F, t16353: F, t16355: F, t16358: F, t16361: F, t16363: F, t16441: F, t16459: F, t16519: F, t16540: F, t16543: F, t16547: F, t16550: F, t16556: F, t1987: F, t240: F, t5423: F, t6857: F, t6876: F) -> (F,) {
    let t18671 = t16353 + t16355 + t16358 + t16361 + t16363 + t16441 - 0.58482233974552040708e0 * t1987 * t16519 - 0.11696446794910408142e1 * t5423 * t6876 - 0.35089340384731224426e1 * t1987 * t16346 + 0.23392893589820816284e1 * t1987 * t16556 + 0.19751789702565206229e-1 * t240 * t16459 - t16540 - t16543 - 0.34631511798751726598e2 * t1987 * t16547 + 0.23392893589820816284e1 * t5423 * t6857 - 0.1025389702100779493e4 * t1987 * t16309 - 0.17315755899375863299e2 * t1987 * t16550 + 0.1038945353962551798e3 * t1987 * t16313;
    (t18671,)
}
