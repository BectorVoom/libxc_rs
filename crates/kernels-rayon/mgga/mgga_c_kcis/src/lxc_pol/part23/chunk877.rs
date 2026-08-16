//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 877/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk877(t11727: f64, t11730: f64, t11736: f64, t1319: f64, t1410: f64, t16194: f64, t16483: f64, t16488: f64, t16491: f64, t16500: f64, t16503: f64, t16530: f64, t1897: f64, t3781: f64, t3809: f64, t3821: f64, t3824: f64, t456: f64, t5481: f64, t5503: f64, t5510: f64) -> f64 {
    let t16533 = 3.0_f64 / 16.0_f64 * t11727 * t16483 - t11730 * t5503 / 4.0_f64 - t3821 * t16488 / 4.0_f64 - t3821 * t16491 / 8.0_f64 + t11736 * t1897 / 4.0_f64 + t3824 * t5481 / 2.0_f64 + t1410 * t16194 / 4.0_f64 - t16500 * t3781 / 8.0_f64 + t16503 * t1319 / 2.0_f64 + t5510 * t3809 / 4.0_f64 + t456 * t16530 / 2.0_f64;
    t16533
}
