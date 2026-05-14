//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 845/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk845<F: Float>(t240: F, t28352: F, t28354: F, t28356: F, t28360: F, t28441: F, t28444: F, t28461: F, t28464: F, t28467: F, t28470: F, t29688: F, t29727: F, t1987: F, t2396: F, t24819: F, t28338: F, t28343: F, t28346: F, t28456: F, t28472: F, t28476: F, t28509: F, t28530: F, t7517: F, t8592: F, t8609: F, t8613: F) -> (F, F) {
    let t29730 = t240 * (t29688 + t29727) + t28352 + t28354 + t28356 - t28360 + t28441 + t28444 - t28461 + t28464 - t28467 + t28470;
    let t29753 = -0.51947267698127589897e2 * t1987 * t28346 - 0.1025389702100779493e4 * t1987 * t28472 + 0.1038945353962551798e3 * t1987 * t28476 - 0.58482233974552040708e0 * t1987 * t28509 + 0.35089340384731224426e1 * t1987 * t28338 - 0.35089340384731224426e1 * t1987 * t28343 - 0.17544670192365612213e1 * t24819 * t2396 - 0.17544670192365612213e1 * t7517 * t8609 - 0.51947267698127589899e2 * t7517 * t8613 + 0.35089340384731224426e1 * t7517 * t8592 - t28530 + 0.19751789702565206229e-1 * t240 * t28456;
    (t29730, t29753)
}
