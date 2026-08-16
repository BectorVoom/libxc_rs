//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 794/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk794(t16320: f64, t8557: f64, t3219: f64, t925: f64, t11854: f64, t16312: f64, t2992: f64, t11472: f64, t11882: f64, t11883: f64, t16288: f64, t16293: f64, t16296: f64, t16298: f64, t16300: f64, t16302: f64, t16306: f64, t16309: f64, t16314: f64, t16317: f64, t1901: f64, t446: f64) -> f64 {
    let t16321 = t8557 * t16320;
    let t16324 = t925 * t3219;
    let t16325 = t11854 * t16324;
    let t16328 = t2992 * t16312;
    let t16329 = t11472 * t16328;
    let t16332 = t446 * t16288 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t16293 - 2.0_f64 / 9.0_f64 * t16296 + 2.0_f64 / 81.0_f64 * t16298 + t16300 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t16302 + t11882 - 8.0_f64 / 81.0_f64 * t11883 - 2.0_f64 / 9.0_f64 * t1901 * t16306 - 4.0_f64 / 9.0_f64 * t1901 * t16309 + 4.0_f64 / 27.0_f64 * t1901 * t16314 + 4.0_f64 / 27.0_f64 * t1901 * t16317 - 2.0_f64 / 9.0_f64 * t1901 * t16321 - 4.0_f64 / 9.0_f64 * t1901 * t16325 - 4.0_f64 / 9.0_f64 * t1901 * t16329;
    t16332
}
