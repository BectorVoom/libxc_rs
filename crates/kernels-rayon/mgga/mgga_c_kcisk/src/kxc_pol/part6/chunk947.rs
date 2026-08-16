//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 947/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk947(t1987: f64, t2396: f64, t240: f64, t24819: f64, t28338: f64, t28343: f64, t28346: f64, t28456: f64, t28472: f64, t28476: f64, t28509: f64, t28530: f64, t7517: f64, t8592: f64, t8609: f64, t8613: f64) -> f64 {
    let t29753 = -0.51947267698127589897e2_f64 * t1987 * t28346 - 0.1025389702100779493e4_f64 * t1987 * t28472 + 0.1038945353962551798e3_f64 * t1987 * t28476 - 0.58482233974552040708e0_f64 * t1987 * t28509 + 0.35089340384731224426e1_f64 * t1987 * t28338 - 0.35089340384731224426e1_f64 * t1987 * t28343 - 0.17544670192365612213e1_f64 * t24819 * t2396 - 0.17544670192365612213e1_f64 * t7517 * t8609 - 0.51947267698127589899e2_f64 * t7517 * t8613 + 0.35089340384731224426e1_f64 * t7517 * t8592 - t28530 + 0.19751789702565206229e-1_f64 * t240 * t28456;
    t29753
}
