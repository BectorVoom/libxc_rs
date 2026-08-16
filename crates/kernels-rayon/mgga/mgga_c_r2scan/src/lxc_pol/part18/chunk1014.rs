//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1014/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1014(t12474: f64, t12509: f64, t12532: f64, t12554: f64, t797: f64, t1048: f64, t499: f64, t11629: f64, t3275: f64, t3582: f64, t11483: f64, t3579: f64) -> (f64, f64, f64, f64, f64) {
    let t12556 = t12474 + t12509 + t12532 + t12554;
    let t12557 = t12556 * t797;
    let t12559 = t1048 * t499 * t12557;
    let t12560 = t12559 / 4.0_f64;
    let t12562 = t3275 * t11629 * t3582;
    let t12563 = 5.0_f64 / 8.0_f64 * t12562;
    let t12564 = t3579 * t11483;
    (t12556, t12557, t12560, t12563, t12564)
}
