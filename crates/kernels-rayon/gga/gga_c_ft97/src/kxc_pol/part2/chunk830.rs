//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 830/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk830(t3436: f64, t8392: f64, t3426: f64, t1986: f64, t920: f64, t2222: f64, t9133: f64, t3431: f64, t1647: f64, t3419: f64, t2210: f64, t11437: f64, t3440: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13040 = 4.0_f64 / 27.0_f64 * t8392 * t3436;
    let t13042 = 2.0_f64 / 27.0_f64 * t8392 * t3426;
    let t13043 = t920 * t1986;
    let t13044 = t2222 * t13043;
    let t13045 = t9133 * t13044;
    let t13049 = 2.0_f64 / 27.0_f64 * t8392 * t3431;
    let t13050 = t3419 * t1647;
    let t13051 = t2210 * t13050;
    let t13054 = t3440 * t11437;
    (t13040, t13042, t13045, t13049, t13051, t13054)
}
