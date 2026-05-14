//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 714/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk714<F: Float>(t1875: F, t9083: F, t4940: F, t8769: F, t5190: F, t116: F, t5294: F, t144: F, t3694: F, t3116: F, t9048: F, t9051: F, t9054: F, t9057: F, t9062: F, t9064: F, t9069: F, t9073: F, t9076: F, t9081: F) -> (F, F) {
    let t9084 = t1875 * t9083;
    let t9085 = t9084 * t4940;
    let t9087 = t1875 * t8769;
    let t9088 = t9087 * t5190;
    let t9090 = t116 * t5294;
    let t9091 = t3694 * t144;
    let t9092 = t9091 * t3116;
    let t9093 = t9090 * t9092;
    let t9095 = 0.56275309320814680968e-8 * t9048 + 0.5627530932081468097e-7 * t9051 + 0.33352499990802834256e-8 * t9054 - 0.17376185052903442709e-3 * t9057 + 0.25782472674694840219e-8 * t9062 - 0.4637672555408563478e-4 * t9064 - 0.42270452978984302532e-6 * t9069 + 0.16882592796244404291e-6 * t9073 + 0.33765185592488808582e-6 * t9076 + 0.16882592796244404291e-6 * t9081 - 0.17376185052903442709e-3 * t9085 + 0.25745714186718600948e-5 * t9088 - 0.17790223495094231792e-8 * t9093;
    (t9092, t9095)
}
