//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 919/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk919<F: Float>(t12474: F, t12509: F, t12532: F, t12554: F, t797: F, t1048: F, t499: F, t11629: F, t3275: F, t3582: F, t11483: F, t3579: F, t106: F, t3052: F, t97: F) -> (F, F, F, F, F, F) {
    let t12556 = t12474 + t12509 + t12532 + t12554;
    let t12557 = t12556 * t797;
    let t12559 = t1048 * t499 * t12557;
    let t12560 = t12559 / 4.0;
    let t12562 = t3275 * t11629 * t3582;
    let t12563 = 5.0 / 8.0 * t12562;
    let t12564 = t3579 * t11483;
    let t12565 = t12564 / 2.0;
    let t12567 = t97 * t106 * t3052;
    (t12556, t12557, t12560, t12563, t12565, t12567)
}
