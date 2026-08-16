//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1103/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1103(t40803: f64, t40831: f64, t118: f64, t305: f64, t326: f64, t40806: f64, t40809: f64, t40814: f64, t40824: f64, t40827: f64, t40834: f64, t43080: f64, t43644: f64, t43749: f64, t43971: f64) -> f64 {
    let t44029 = 0.3193131120497015617e0_f64 * t40803;
    let t44035 = 0.3193131120497015617e0_f64 * t40831;
    let t44043 = -0.79828278012425390428e-1_f64 * t118 * t43971 - t44029 - 0.47896966807455234256e0_f64 * t40806 - 0.17961362552795712846e0_f64 * t40809 - 0.2993560425465952141e-1_f64 * t40814 - 0.35922725105591425692e0_f64 * t40824 - 0.11974241701863808564e0_f64 * t40827 + t44035 - 0.35922725105591425692e0_f64 * t40834 + 0.59871208509319042821e-1_f64 * t305 * t43080 - 0.11974241701863808564e0_f64 * t326 * t43644 - 0.59871208509319042821e-1_f64 * t326 * t43749;
    t44043
}
