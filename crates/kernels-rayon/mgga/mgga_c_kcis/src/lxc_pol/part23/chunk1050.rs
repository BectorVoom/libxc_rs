//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1050/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1050(t27479: f64, t303: f64, t2244: f64, t3245: f64, t110: f64, t2238: f64, t2237: f64, t27342: f64, t27416: f64, t27455: f64, t27459: f64, t27462: f64, t27465: f64, t27471: f64, t27477: f64, t7898: f64, t7908: f64, t7911: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27480 = t303 * t27479;
    let t27482 = t3245 * t2244;
    let t27483 = 0.55273148148148148147e-3_f64 * t27482;
    let t27484 = t110 * t2238;
    let t27486 = 0.15445601851851851852e-3_f64 * t2237 * t27484;
    let t27487 = 0.46336805555555555556e-3_f64 * t7908 * t27455 - 0.46336805555555555556e-3_f64 * t27459 * t7911 + 0.33163888888888888888e-2_f64 * t27462 + 0.24872916666666666666e-2_f64 * t27465 + 0.69505208333333333333e-3_f64 * t2237 * t27416 - 0.13901041666666666667e-2_f64 * t2237 * t27342 + 0.61836467013888888889e-4_f64 * t27471 - 0.2782641015625e-3_f64 * t7898 * t27342 - 0.49745833333333333332e-2_f64 * t27477 + 0.33163888888888888888e-2_f64 * t27480 - t27483 + t27486;
    (t27480, t27482, t27483, t27484, t27486, t27487)
}
