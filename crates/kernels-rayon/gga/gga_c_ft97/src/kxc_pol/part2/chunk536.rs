//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 536/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk536(t3000: f64, t3330: f64, t89: f64, t1018: f64, t375: f64, t1017: f64, t358: f64, t363: f64, t1969: f64, t446: f64, t1984: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3332 = t89 * t3000 * t3330;
    let t3335 = t89 * t375 * t1018;
    let t3337 = t1017 * t358;
    let t3338 = t3337 * t363;
    let t3339 = t1969 * t3338;
    let t3340 = t446 * t3339;
    let t3342 = t1984 * t1017;
    let t3343 = t3342 * t558;
    (t3332, t3335, t3337, t3338, t3339, t3340, t3342, t3343)
}
