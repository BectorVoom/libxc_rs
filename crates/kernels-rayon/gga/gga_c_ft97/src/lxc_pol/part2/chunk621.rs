//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 621/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk621(t524: f64, t173: f64, t322: f64, t674: f64, t797: f64, t2252: f64, t342: f64, t344: f64, t422: f64, t1526: f64, t1529: f64, t1533: f64, t630: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7367 = t524 * t524;
    let t7368 = 1.0_f64 / t7367;
    let t7512 = t173 * t322;
    let t7513 = t674 * t674;
    let t7514 = 1.0_f64 / t7513;
    let t7639 = t797 * t797;
    let t7640 = 1.0_f64 / t7639;
    let t7704 = t342 * t2252 * t344 / 18.0_f64;
    let t7705 = t173 * t422;
    let t7707 = t1526 * t7705 * t1529;
    let t7710 = t342 * t630 * t1533;
    (t7368, t7512, t7514, t7640, t7704, t7705, t7707, t7710)
}
