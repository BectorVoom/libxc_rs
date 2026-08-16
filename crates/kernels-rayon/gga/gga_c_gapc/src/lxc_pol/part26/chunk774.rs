//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 774/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk774(t144: f64, t3694: f64, t3116: f64, t9090: f64, t9048: f64, t9051: f64, t9054: f64, t9057: f64, t9062: f64, t9064: f64, t9069: f64, t9073: f64, t9076: f64, t9081: f64, t9085: f64, t9088: f64) -> (f64, f64) {
    let t9091 = t3694 * t144;
    let t9092 = t9091 * t3116;
    let t9093 = t9090 * t9092;
    let t9095 = 0.56275309320814680968e-8_f64 * t9048 + 0.5627530932081468097e-7_f64 * t9051 + 0.33352499990802834256e-8_f64 * t9054 - 0.17376185052903442709e-3_f64 * t9057 + 0.25782472674694840219e-8_f64 * t9062 - 0.4637672555408563478e-4_f64 * t9064 - 0.42270452978984302532e-6_f64 * t9069 + 0.16882592796244404291e-6_f64 * t9073 + 0.33765185592488808582e-6_f64 * t9076 + 0.16882592796244404291e-6_f64 * t9081 - 0.17376185052903442709e-3_f64 * t9085 + 0.25745714186718600948e-5_f64 * t9088 - 0.17790223495094231792e-8_f64 * t9093;
    (t9092, t9095)
}
