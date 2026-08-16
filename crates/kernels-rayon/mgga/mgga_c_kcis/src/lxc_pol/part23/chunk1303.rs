//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1303/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1303(t27567: f64, t99291: f64, t11425: f64, t1616: f64, t28788: f64, t7974: f64, t12617: f64, t16685: f64, t16694: f64, t27583: f64, t28701: f64, t28758: f64, t28805: f64, t3797: f64, t6151: f64, t6159: f64, t94931: f64, t95088: f64, t98543: f64, t98553: f64, t99224: f64, t99419: f64) -> f64 {
    let t99437 = 0.10306077835648148148e-4_f64 * t27567 * t99291;
    let t99446 = t1616 * t11425;
    let t99452 = 0.23168402777777777778e-3_f64 * t28788 * t7974;
    let t99461 = t99437 + 0.11584201388888888889e-3_f64 * t27583 * t99224 - 0.46377350260416666666e-4_f64 * t27567 * t99419 + 0.23168402777777777778e-3_f64 * t27583 * t6159 * t28758 * t16685 + 0.92673611111111111112e-3_f64 * t27583 * t6151 * t99446 * t16694 - t99452 + 0.15445601851851851852e-3_f64 * t27583 * t12617 * t28805 * t3797 + 0.30918233506944444444e-4_f64 * t94931 * t28701 - 0.38691203703703703703e-3_f64 * t98543 - 0.11607361111111111111e-2_f64 * t98553 + t95088;
    t99461
}
