//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 911/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk911<F: Float>(t44: F, t1048: F, t795: F, t8601: F, t2999: F, t4904: F, t1212: F, t3002: F, t472: F, t8571: F, t1217: F, t2509: F, t415: F, t3007: F, t4920: F, t1224: F, t3010: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t8603 = t1048 * t8601 * t795;
    let t8604 = t4904 * t2999;
    let t8609 = t1212 * t3002;
    let t8612 = t472 * t8571;
    let t8615 = piecewise3(t45, 0.0, 8.0 / 27.0 * t8604 * t415 - 8.0 / 9.0 * t2509 * t1217 - 2.0 / 9.0 * t8609 * t415 + 2.0 / 3.0 * t8612);
    let t8616 = t4920 * t3007;
    let t8621 = t1224 * t3010;
    (t8603, t8604, t8615, t8616, t8621)
}
