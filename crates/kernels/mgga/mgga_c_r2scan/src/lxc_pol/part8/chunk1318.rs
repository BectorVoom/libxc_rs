//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1318/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1318<F: Float>(t44: F, t19064: F, t23795: F, t2266: F, t31402: F, t910: F, t8601: F, t9577: F, t2: F, t464: F, t9904: F, t1212: F, t1217: F, t19347: F, t2509: F, t32155: F, t32158: F, t32168: F, t415: F, t472: F, t7059: F, t7062: F, t8571: F, t8604: F, t9858: F, t9864: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t32218 = 0.35089341735807877242e1 * t19064;
    let t32219 = 0.17544670867903938621e1 * t23795;
    let t32222 = 9.0 * t2266 * t31402 * t910;
    let t32225 = 9.0 * t2266 * t8601 * t9577;
    let t32227 = t9904 * t2 * t464;
    let t32228 = 0.18311447306006545054e-3 * t32227;
    let t32246 = piecewise3(t45, 0.0, -56.0 / 81.0 * t19347 * t9858 * t415 + 16.0 / 9.0 * t8604 * t1217 + 8.0 / 9.0 * t7059 * t32155 - 4.0 / 3.0 * t7062 * t32158 - 2.0 / 3.0 * t2509 * t8571 - 2.0 / 9.0 * t1212 * t9864 * t415 + 2.0 / 3.0 * t472 * t32168);
    (t32218, t32219, t32222, t32225, t32228, t32246)
}
