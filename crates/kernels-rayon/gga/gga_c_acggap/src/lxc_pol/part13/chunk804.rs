//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 804/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk804(t1967: f64, t2327: f64, t7429: f64, t7434: f64, t7441: f64, t7448: f64, t7463: f64, t8704: f64, t8706: f64, t8708: f64, t8710: f64, t8712: f64, t8714: f64, t8716: f64, t8718: f64, t8720: f64) -> f64 {
    let t8722 = t1967 * t2327;
    let t8728 = 0.34299214494455789578e-2_f64 * t8704 - 0.85748036236139473944e-3_f64 * t8706 - 0.34299214494455789578e-2_f64 * t8708 + 0.17149607247227894789e-2_f64 * t8710 + 0.40015750243531754507e-2_f64 * t8712 - 0.40015750243531754507e-2_f64 * t8714 + 0.80031500487063509015e-2_f64 * t8716 - 0.17149607247227894789e-2_f64 * t8718 - 0.17149607247227894789e-2_f64 * t8720 - 0.64311027177104605458e-3_f64 * t8722 - 0.47172138434406228102e-3_f64 * t7429 - 0.94344276868812456204e-3_f64 * t7434 - 0.28015625e-1_f64 * t7441 - 0.420234375e-1_f64 * t7448 - t7463;
    t8728
}
