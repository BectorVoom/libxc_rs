//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1052/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1052(t39338: f64, t34960: f64, t37297: f64, t39330: f64, t39333: f64, t39341: f64, t39345: f64, t39350: f64, t39355: f64, t39360: f64, t39362: f64, t39367: f64, t39370: f64, t39374: f64, t39379: f64, t39384: f64, t5928: f64, t8042: f64) -> f64 {
    let t42928 = 0.60975299583150056624e-3_f64 * t39338;
    let t42944 = 0.212822999466489197e-4_f64 * t39330 + 0.162600798888400151e-2_f64 * t39333 - t42928 + 0.68400385060046895e-6_f64 * t39341 + 0.68400385060046895e-6_f64 * t39345 + 0.1702583995731913576e-4_f64 * t39350 - 0.2553875993597870364e-4_f64 * t39355 + 0.638468998399467591e-4_f64 * t39360 - 0.68186654135613354325e-2_f64 * t39362 - 0.5854073720911195298e0_f64 * t34960 - 0.2363e1_f64 * t37297 + 0.60975299583150056624e-3_f64 * t39367 - 0.16163010989689081288e-5_f64 * t39370 + 0.13637330827122670865e-1_f64 * t39374 + 0.15323255961587222184e-3_f64 * t39379 - 0.20431007948782962912e-3_f64 * t39384 - 0.11974241701863808564e0_f64 * t5928 * t8042;
    t42944
}
