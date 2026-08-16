//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 798/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk798(t36293: f64, t739: f64, t36247: f64, t35979: f64, t4044: f64, t212: f64, t3076: f64, t672: f64, t678: f64, t7901: f64, t7922: f64, t7928: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36998 = t739 * t36293;
    let t37000 = t739 * t36247;
    let t37006 = t4044 * t35979;
    let t37017 = t672 * t212 * t3076 * t678;
    let t37018 = 0.14345846630704086612e-3_f64 * t37017;
    let t37031 = 0.43905552906833964735e0_f64 * t7901;
    let t37039 = 0.9931739975102829193e-4_f64 * t7922;
    let t37041 = 0.24390119833260022651e-2_f64 * t7928;
    (t36998, t37000, t37006, t37018, t37031, t37039, t37041)
}
