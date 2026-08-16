//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1359/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1359(t1380: f64, t20916: f64, t27370: f64, t102038: f64, t102041: f64, t103073: f64, t27369: f64, t28369: f64, t28388: f64, t28392: f64, t28443: f64, t28485: f64, t28489: f64, t28495: f64, t28551: f64, t98155: f64, t98193: f64) -> (f64, f64) {
    let t103289 = t27370 * t20916 * t1380;
    let t103292 = -0.12356481481481481481e-2_f64 * t28392 * t28443 - 0.61890573922526041667e-5_f64 * t28388 * t103073 + 0.46336805555555555557e-3_f64 * t28369 * t28443 - 0.36848765432098765431e-3_f64 * t102038 + 0.33163888888888888888e-2_f64 * t102041 - 0.12356481481481481482e-2_f64 * t28392 * t28551 - 0.12356481481481481482e-2_f64 * t28392 * t28485 - 0.24712962962962962964e-2_f64 * t28392 * t28489 - 0.16489724537037037038e-3_f64 * t98155 * t28485 + 0.16475308641975308643e-2_f64 * t28392 * t28495 - 0.58958024691358024689e-2_f64 * t98193 - 0.92754700520833333333e-4_f64 * t27369 * t103289;
    (t103289, t103292)
}
