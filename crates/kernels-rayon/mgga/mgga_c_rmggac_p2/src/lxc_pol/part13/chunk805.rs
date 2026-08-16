//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 805/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk805(t2231: f64, t934: f64, t36504: f64, t36527: f64, t1347: f64, t2232: f64, t4793: f64, t703: f64, t275: f64, t8198: f64, t36700: f64, t36752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37950 = t934 * t2231;
    let t37964 = 0.13659505348792789029e1_f64 * t36504;
    let t37976 = 0.2439011983326002265e-2_f64 * t36527;
    let t38029 = t1347 * t2232;
    let t38031 = t4793 * t703;
    let t38036 = t275 * t8198;
    let t38047 = 0.18292589874945016987e-2_f64 * t36700;
    let t38060 = 0.30487649791575028312e-3_f64 * t36752;
    (t37950, t37964, t37976, t38029, t38031, t38036, t38047, t38060)
}
