//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 903/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk903<F: Float>(t1201: F, t2107: F, t25786: F, t30668: F, t30670: F, t30674: F, t30707: F, t30716: F, t30731: F, t30734: F, t5765: F, t7804: F, t7821: F, t7825: F, t30665: F) -> (F,) {
    let t30737 = -t30668 - 0.1025389702100779493e4 * t1201 * t30670 + 0.1038945353962551798e3 * t1201 * t30674 - 0.58482233974552040708e0 * t1201 * t30707 - 0.17544670192365612213e1 * t25786 * t2107 - 0.17544670192365612213e1 * t5765 * t7821 - 0.51947267698127589899e2 * t5765 * t7825 + 0.35089340384731224426e1 * t1201 * t30716 - t30731 + 0.35089340384731224426e1 * t5765 * t7804 - 0.51947267698127589897e2 * t1201 * t30734;
    let t30738 = t30665 + t30737;
    (t30738,)
}
