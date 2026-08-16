//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1019/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1019(t30729: f64, t321: f64, t25668: f64, t6560: f64, t1201: f64, t2107: f64, t25786: f64, t30668: f64, t30670: f64, t30674: f64, t30707: f64, t30716: f64, t5765: f64, t7804: f64, t7821: f64, t7825: f64) -> (f64, f64, f64) {
    let t30731 = 0.62182e-1_f64 * t30729 * t321;
    let t30734 = t25668 * t6560;
    let t30737 = -t30668 - 0.1025389702100779493e4_f64 * t1201 * t30670 + 0.1038945353962551798e3_f64 * t1201 * t30674 - 0.58482233974552040708e0_f64 * t1201 * t30707 - 0.17544670192365612213e1_f64 * t25786 * t2107 - 0.17544670192365612213e1_f64 * t5765 * t7821 - 0.51947267698127589899e2_f64 * t5765 * t7825 + 0.35089340384731224426e1_f64 * t1201 * t30716 - t30731 + 0.35089340384731224426e1_f64 * t5765 * t7804 - 0.51947267698127589897e2_f64 * t1201 * t30734;
    (t30731, t30734, t30737)
}
