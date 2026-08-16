//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1309/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1309(t7968: f64, t99059: f64, t98794: f64, t95157: f64, t98767: f64, t98781: f64, t98784: f64, t98787: f64, t98790: f64, t98797: f64, t98800: f64, t98804: f64) -> f64 {
    let t99610 = 0.92754700520833333333e-4_f64 * t7968 * t99059;
    let t99615 = 0.10317654320987654321e-2_f64 * t98794;
    let t99619 = 0.17411041666666666666e-2_f64 * t98767 - 0.7722800925925925926e-4_f64 * t95157 - t99610 - 0.11607361111111111111e-2_f64 * t98781 - 0.19345601851851851852e-2_f64 * t98784 + 0.77382407407407407407e-3_f64 * t98787 + 0.12897067901234567901e-2_f64 * t98790 + t99615 - 0.15476481481481481481e-2_f64 * t98797 + 0.77382407407407407406e-3_f64 * t98800 + 0.38691203703703703703e-3_f64 * t98804;
    t99619
}
