//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 947/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk947<F: Float>(t1987: F, t2396: F, t240: F, t24819: F, t28338: F, t28343: F, t28346: F, t28456: F, t28472: F, t28476: F, t28509: F, t28530: F, t7517: F, t8592: F, t8609: F, t8613: F) -> F {
    let t29753 = -F::cast_from(0.51947267698127589897e2_f64) * t1987 * t28346 - F::cast_from(0.1025389702100779493e4_f64) * t1987 * t28472 + F::cast_from(0.1038945353962551798e3_f64) * t1987 * t28476 - F::cast_from(0.58482233974552040708e0_f64) * t1987 * t28509 + F::cast_from(0.35089340384731224426e1_f64) * t1987 * t28338 - F::cast_from(0.35089340384731224426e1_f64) * t1987 * t28343 - F::cast_from(0.17544670192365612213e1_f64) * t24819 * t2396 - F::cast_from(0.17544670192365612213e1_f64) * t7517 * t8609 - F::cast_from(0.51947267698127589899e2_f64) * t7517 * t8613 + F::cast_from(0.35089340384731224426e1_f64) * t7517 * t8592 - t28530 + F::cast_from(0.19751789702565206229e-1_f64) * t240 * t28456;
    t29753
}
