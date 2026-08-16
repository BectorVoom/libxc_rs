//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 485/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk485(t2409: f64, t799: f64, t779: f64, t1002: f64, t1015: f64, t2248: f64, t2254: f64, t2320: f64, t2333: f64, t2337: f64, t2344: f64, t2348: f64, t2354: f64, t2357: f64, t2360: f64, t2364: f64, t2369: f64, t2377: f64, t2380: f64, t355: f64, t364: f64, t984: f64, t989: f64) -> (f64, f64, f64) {
    let t2410 = t2409 * t799;
    let t2412 = 1.0_f64 * t779 * t2410;
    let t2413 = -50.0_f64 / 3.0_f64 * t2248 * t1015 + 100.0_f64 / 81.0_f64 * t2254 + t2320 * t364 / 2.0_f64 + 20000.0_f64 / 81.0_f64 * t2333 * t2337 + 44.0_f64 / 9.0_f64 * t355 * t2344 - 8.0_f64 / 9.0_f64 * t2348 - t2354 - 8.0_f64 / 3.0_f64 * t984 * t989 + t2357 / 3.0_f64 + t2360 * t1002 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t2364 * t1002 + t2369 / 9.0_f64 - t2377 + t2380 + t2412;
    (t2410, t2412, t2413)
}
