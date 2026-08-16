//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 412/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk412(t147: f64, t184: f64, t2299: f64, t21: f64, t648: f64, t363: f64, t649: f64, t1580: f64, t185: f64, t2236: f64, t2240: f64, t5: f64, t620: f64, t623: f64, t650: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t2300 = t2299 * t184;
    let t2301 = t2300 * t21;
    let t2304 = t648 * t648;
    let t2305 = t2304 * t184;
    let t2306 = t2305 * t21;
    let t2309 = t649 * t363;
    let t2316 = piecewise3(t148, 0.0_f64, t5 * t2236 * t21 / 4.0_f64 + t2240 * t650 / 2.0_f64 + t5 * t620 * t363 / 2.0_f64 + t623 * t2301 / 4.0_f64 + t623 * t2306 / 4.0_f64 + t623 * t2309 / 2.0_f64 + t5 * t185 * t1580 / 4.0_f64);
    (t2300, t2301, t2304, t2305, t2306, t2309, t2316)
}
