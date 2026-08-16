//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 466/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk466(t1026: f64, t1882: f64, t1060: f64, t379: f64, t569: f64, t616: f64, t925: f64, t167: f64, t3052: f64, t1901: f64, t2164: f64, t2195: f64, t3281: f64, t3421: f64, t3426: f64, t3431: f64, t3436: f64, t3442: f64, t3447: f64, t3452: f64, t3457: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t3460 = t1882 * t1026;
    let t3463 = t569 * t1060 * t379;
    let t3467 = t569 * t616 * t925;
    let t3471 = t569 * t167 * t3052;
    let t3474 = t2195 / 27.0_f64 + t1901 * t3421 / 9.0_f64 + t1901 * t3426 / 9.0_f64 + t1901 * t3431 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t3436 - 2.0_f64 / 27.0_f64 * t1901 * t3442 + t1901 * t3447 / 9.0_f64 + t2164 + 2.0_f64 / 3.0_f64 * t446 * t3452 + t446 * t3457 / 3.0_f64 + t3460 / 27.0_f64 - t446 * t3463 / 9.0_f64 - t446 * t3467 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3281 * t3471;
    (t3460, t3463, t3467, t3471, t3474)
}
