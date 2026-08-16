//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 359/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk359(t1882: f64, t449: f64, t104: f64, t1637: f64, t89: f64, t454: f64, t494: f64, t432: f64, t452: f64, t499: f64, t110: f64, t1755: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1883 = t1882 * t449;
    let t1887 = 4.0_f64 / 27.0_f64 * t89 * t1637 * t104;
    let t1888 = t1882 * t454;
    let t1890 = t1882 * t494;
    let t1893 = t452 * t499 * t432;
    let t1897 = t452 * t110 * t1755;
    (t1883, t1887, t1888, t1890, t1893, t1897)
}
