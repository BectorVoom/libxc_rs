//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 885/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk885<F: Float>(t1264: F, t20552: F, t2143: F, t26341: F, t30404: F, t30437: F, t30442: F, t30445: F, t30448: F, t30452: F, t30465: F, t361: F, t4031: F, t4081: F, t6095: F, t7928: F, t7978: F, t7995: F) -> (F,) {
    let t30474 = 0.1038945353962551798e3 * t1264 * t30404 - 0.58482233974552040708e0 * t1264 * t30437 - 6.0 * t20552 * t7928 + 6.0 * t4081 * t30442 - 6.0 * t4031 * t30445 + 0.48245472966453314466e2 * t4081 * t30448 - 0.51947267698127589897e2 * t1264 * t30452 - 0.62182e-1 * t30465 * t361 - 0.17544670192365612213e1 * t26341 * t2143 - 0.17544670192365612213e1 * t6095 * t7995 + 0.35089340384731224426e1 * t6095 * t7978;
    (t30474,)
}
