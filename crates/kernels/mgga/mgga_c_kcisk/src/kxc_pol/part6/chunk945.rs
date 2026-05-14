//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 945/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk945<F: Float>(t1550: F, t2107: F, t240: F, t27491: F, t30553: F, t30656: F, t30670: F, t30674: F, t30707: F, t30716: F, t30731: F, t30734: F, t31576: F, t31606: F, t6568: F, t7821: F, t7825: F) -> (F,) {
    let t31613 = 0.19751789702565206229e-1 * t240 * t30656 - 0.35089340384731224426e1 * t1550 * t30553 + 0.35089340384731224426e1 * t1550 * t30716 - 0.17544670192365612213e1 * t6568 * t7821 - 0.51947267698127589899e2 * t6568 * t7825 - t30731 - 0.1025389702100779493e4 * t1550 * t30670 + 0.1038945353962551798e3 * t1550 * t30674 - 0.58482233974552040708e0 * t1550 * t30707 + t240 * (t31576 + t31606) - 0.17544670192365612213e1 * t27491 * t2107 - 0.51947267698127589897e2 * t1550 * t30734;
    (t31613,)
}
