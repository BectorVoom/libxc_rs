//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 719/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk719(t1712: f64, t374: f64, t930: f64, t401: f64, t428: f64, t3057: f64, t1685: f64, t25: f64, t3099: f64, t3066: f64, t1655: f64, t373: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11332 = t374 * t930 * t1712;
    let t11335 = t401 * t428;
    let t11339 = t3057 * t428;
    let t11340 = t374 * t11339;
    let t11343 = t930 * t1685;
    let t11344 = t374 * t11343;
    let t11347 = t3099 * t25;
    let t11348 = t11347 * t3066;
    let t11351 = t373 * t1655;
    (t11332, t11335, t11340, t11344, t11348, t11351)
}
