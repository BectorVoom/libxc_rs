//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 817/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk817(t44: f64, t1048: f64, t795: f64, t8601: f64, t2999: f64, t4904: f64, t1212: f64, t3002: f64, t472: f64, t8571: f64, t1217: f64, t2509: f64, t415: f64, zeta_threshold: f64) -> (f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t8603 = t1048 * t8601 * t795;
    let t8604 = t4904 * t2999;
    let t8609 = t1212 * t3002;
    let t8612 = t472 * t8571;
    let t8615 = piecewise3(t45, 0.0_f64, 8.0_f64 / 27.0_f64 * t8604 * t415 - 8.0_f64 / 9.0_f64 * t2509 * t1217 - 2.0_f64 / 9.0_f64 * t8609 * t415 + 2.0_f64 / 3.0_f64 * t8612);
    (t8603, t8615)
}
