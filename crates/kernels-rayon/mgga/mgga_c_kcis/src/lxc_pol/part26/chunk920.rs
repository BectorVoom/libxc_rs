//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 920/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk920(t21484: f64, t833: f64, t16963: f64, t16831: f64, t531: f64, t1380: f64, t16969: f64, t12135: f64, t1368: f64, t16830: f64, t16925: f64, t16935: f64, t16940: f64, t16944: f64, t16954: f64, t16981: f64, t21470: f64, t21474: f64, t21478: f64, t21480: f64) -> f64 {
    let t21485 = t21484 * t833;
    let t21486 = t16963 * t21485;
    let t21489 = t16831 * t531;
    let t21491 = t21489 * t21484 * t1380;
    let t21494 = t16969 * t21485;
    let t21497 = -t1368 * t21470 / 16.0_f64 + t1368 * t21474 / 24.0_f64 - t21478 / 432.0_f64 - t21480 / 162.0_f64 - t12135 / 1296.0_f64 - t16925 - t16935 - t16940 + t16944 + t16954 / 81.0_f64 - t16981 - t16830 * t21486 / 108.0_f64 + t16830 * t21491 / 72.0_f64 + t16830 * t21494 / 72.0_f64;
    t21497
}
