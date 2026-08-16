//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1259/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1259(t27345: f64, t8151: f64, t27348: f64, t28544: f64, t1014: f64, t28406: f64, t27459: f64, t28373: f64, t28495: f64, t3805: f64, t3984: f64, t7908: f64, t94586: f64, t94589: f64, t94592: f64, t94594: f64, t94602: f64) -> (f64, f64) {
    let t98566 = t8151 * t27345;
    let t98568 = t8151 * t27348;
    let t98570 = t28544 * t27348;
    let t98573 = t1014 * t28406;
    let t98574 = 0.88437037037037037034e-2_f64 * t98573;
    let t98581 = 0.20612155671296296296e-4_f64 * t94586 + t94589 - 0.61890573922526041668e-5_f64 * t94592 + 0.11054629629629629629e-2_f64 * t94594 - 0.12356481481481481481e-2_f64 * t98566 - 0.12356481481481481481e-2_f64 * t98568 - 0.16489724537037037037e-3_f64 * t98570 - 0.23168402777777777778e-3_f64 * t94602 + t98574 - 0.61782407407407407408e-3_f64 * t27459 * t28495 + 0.23168402777777777778e-3_f64 * t7908 * t3984 * t28373 * t3805;
    (t98573, t98581)
}
