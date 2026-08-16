//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1072/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1072(t1550: f64, t2107: f64, t240: f64, t27491: f64, t30553: f64, t30656: f64, t30670: f64, t30674: f64, t30707: f64, t30716: f64, t30731: f64, t30734: f64, t31576: f64, t31606: f64, t6568: f64, t7821: f64, t7825: f64) -> f64 {
    let t31613 = 0.19751789702565206229e-1_f64 * t240 * t30656 - 0.35089340384731224426e1_f64 * t1550 * t30553 + 0.35089340384731224426e1_f64 * t1550 * t30716 - 0.17544670192365612213e1_f64 * t6568 * t7821 - 0.51947267698127589899e2_f64 * t6568 * t7825 - t30731 - 0.1025389702100779493e4_f64 * t1550 * t30670 + 0.1038945353962551798e3_f64 * t1550 * t30674 - 0.58482233974552040708e0_f64 * t1550 * t30707 + t240 * (t31576 + t31606) - 0.17544670192365612213e1_f64 * t27491 * t2107 - 0.51947267698127589897e2_f64 * t1550 * t30734;
    t31613
}
