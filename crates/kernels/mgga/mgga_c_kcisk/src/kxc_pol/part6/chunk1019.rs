//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1019/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1019<F: Float>(t30729: F, t321: F, t25668: F, t6560: F, t1201: F, t2107: F, t25786: F, t30668: F, t30670: F, t30674: F, t30707: F, t30716: F, t5765: F, t7804: F, t7821: F, t7825: F) -> (F, F, F) {
    let t30731 = F::new(0.62182e-1) * t30729 * t321;
    let t30734 = t25668 * t6560;
    let t30737 = -t30668 - F::new(0.1025389702100779493e4) * t1201 * t30670 + F::new(0.1038945353962551798e3) * t1201 * t30674 - F::new(0.58482233974552040708e0) * t1201 * t30707 - F::new(0.17544670192365612213e1) * t25786 * t2107 - F::new(0.17544670192365612213e1) * t5765 * t7821 - F::new(0.51947267698127589899e2) * t5765 * t7825 + F::new(0.35089340384731224426e1) * t1201 * t30716 - t30731 + F::new(0.35089340384731224426e1) * t5765 * t7804 - F::new(0.51947267698127589897e2) * t1201 * t30734;
    (t30731, t30734, t30737)
}
