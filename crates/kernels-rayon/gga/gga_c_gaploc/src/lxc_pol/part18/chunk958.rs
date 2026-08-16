//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 958/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk958(t10427: f64, t123: f64, t2754: f64, t883: f64, t2488: f64, t2487: f64, t2465: f64, t2787: f64, t2464: f64, t2778: f64, t587: f64, t1407: f64, t3396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10428 = 0.19171462976960374838e0_f64 * t10427;
    let t10429 = t2754 * t123;
    let t10430 = t10429 * t883;
    let t10431 = t2488 * t10430;
    let t10432 = t2487 * t10431;
    let t10433 = 0.19171462976960374838e0_f64 * t10432;
    let t10434 = t2465 * t2787;
    let t10435 = t2464 * t10434;
    let t10436 = t2487 * t10435;
    let t10437 = 0.42603251059911944084e-1_f64 * t10436;
    let t10438 = t2465 * t2778;
    let t10439 = t2464 * t10438;
    let t10440 = t587 * t10439;
    let t10441 = 0.42603251059911944084e-1_f64 * t10440;
    let t10442 = t1407 * t3396;
    (t10428, t10429, t10430, t10431, t10433, t10434, t10435, t10437, t10438, t10439, t10441, t10442)
}
