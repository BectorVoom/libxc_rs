//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2290/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2290(t19356: f64, t19444: f64, t12568: f64, t12571: f64, t1437: f64, t19297: f64, t19299: f64, t19310: f64, t19313: f64, t19318: f64, t2235: f64, t2240: f64, t3953: f64, t3958: f64, t4021: f64, t5389: f64, t5445: f64, t605: f64, t645: f64, t86: f64, t9231: f64, t9239: f64) -> (f64, f64) {
    let t19445 = t19356 + t19444;
    let t19448 = -8.0_f64 * t12568 * t1437 + 40.0_f64 * t12571 * t3958 + t19297 * t86 - 4.0_f64 * t19299 * t645 - 120.0_f64 * t19310 * t9239 + 40.0_f64 * t19313 * t2240 + 20.0_f64 * t19318 * t2240 - 4.0_f64 * t19445 * t605 - 4.0_f64 * t2235 * t5445 - 8.0_f64 * t3953 * t4021 + 20.0_f64 * t5389 * t9231;
    (t19445, t19448)
}
