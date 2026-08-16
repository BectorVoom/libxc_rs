//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 806/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk806(t1045: f64, t2178: f64, t2180: f64, t144: f64, t1882: f64, t3584: f64, t3580: f64, t3571: f64, t3442: f64, t8392: f64, t3420: f64, t9099: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12664 = t1045 * t2178;
    let t12665 = t12664 * t2180;
    let t12666 = t144 * t12665;
    let t12670 = 2.0_f64 / 9.0_f64 * t1882 * t3584;
    let t12672 = 2.0_f64 / 9.0_f64 * t1882 * t3580;
    let t12674 = 2.0_f64 / 9.0_f64 * t1882 * t3571;
    let t12676 = 4.0_f64 / 81.0_f64 * t8392 * t3442;
    let t12677 = t9099 * t3420;
    (t12665, t12666, t12670, t12672, t12674, t12676, t12677)
}
