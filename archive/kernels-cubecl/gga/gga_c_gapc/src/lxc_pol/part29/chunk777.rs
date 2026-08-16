//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 777/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk777<F: Float>(t144: F, t3694: F, t3116: F, t9090: F, t9048: F, t9051: F, t9054: F, t9057: F, t9062: F, t9064: F, t9069: F, t9073: F, t9076: F, t9081: F, t9085: F, t9088: F) -> (F, F) {
    let t9091 = t3694 * t144;
    let t9092 = t9091 * t3116;
    let t9093 = t9090 * t9092;
    let t9095 = F::cast_from(0.56275309320814680968e-8_f64) * t9048 + F::cast_from(0.5627530932081468097e-7_f64) * t9051 + F::cast_from(0.33352499990802834256e-8_f64) * t9054 - F::cast_from(0.17376185052903442709e-3_f64) * t9057 + F::cast_from(0.25782472674694840219e-8_f64) * t9062 - F::cast_from(0.4637672555408563478e-4_f64) * t9064 - F::cast_from(0.42270452978984302532e-6_f64) * t9069 + F::cast_from(0.16882592796244404291e-6_f64) * t9073 + F::cast_from(0.33765185592488808582e-6_f64) * t9076 + F::cast_from(0.16882592796244404291e-6_f64) * t9081 - F::cast_from(0.17376185052903442709e-3_f64) * t9085 + F::cast_from(0.25745714186718600948e-5_f64) * t9088 - F::cast_from(0.17790223495094231792e-8_f64) * t9093;
    (t9092, t9095)
}
