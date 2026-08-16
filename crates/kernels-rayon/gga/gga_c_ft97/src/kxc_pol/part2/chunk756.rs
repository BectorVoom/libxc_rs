//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 756/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk756(t1647: f64, t447: f64, t986: f64, t1882: f64, t3210: f64, t8232: f64, t951: f64, t1755: f64, t452: f64, t3216: f64, t3291: f64, t432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11878 = t447 * t986 * t1647;
    let t11882 = 2.0_f64 / 27.0_f64 * t1882 * t3210;
    let t11883 = t8232 * t951;
    let t11887 = t452 * t986 * t1755;
    let t11897 = 2.0_f64 / 9.0_f64 * t1882 * t3216;
    let t11899 = t452 * t3291 * t432;
    (t11878, t11882, t11883, t11887, t11897, t11899)
}
