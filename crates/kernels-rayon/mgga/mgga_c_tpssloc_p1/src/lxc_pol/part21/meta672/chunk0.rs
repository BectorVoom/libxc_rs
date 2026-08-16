//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2476/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2476(t3540: f64, t3567: f64, t374: f64, t485: f64, t486: f64, t9697: f64, t11820: f64, t3536: f64, t1229: f64, t204: f64, t1090: f64, t1227: f64, t248: f64) -> (f64, f64, f64, f64, f64) {
    let t45224 = t3567 * t3540;
    let t45250 = 7.0_f64 / 31104.0_f64 * t485 * t374 * t9697 * t486;
    let t45266 = t3536 * t11820;
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    (t45224, t45250, t45266, t45293, t45296)
}
