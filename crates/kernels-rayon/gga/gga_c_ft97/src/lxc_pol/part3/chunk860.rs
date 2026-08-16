//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 860/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk860(t3483: f64, t925: f64, t13220: f64, t11593: f64, t13040: f64, t13042: f64, t13049: f64, t13062: f64, t13075: f64, t13084: f64, t17195: f64, t17200: f64, t17204: f64, t17208: f64, t17357: f64, t17360: f64, t17362: f64, t17366: f64, t1901: f64, t446: f64) -> f64 {
    let t17369 = t925 * t3483;
    let t17370 = t13220 * t17369;
    let t17373 = 2.0_f64 / 9.0_f64 * t1901 * t17195 + t1901 * t17200 / 9.0_f64 - 10.0_f64 / 81.0_f64 * t1901 * t17204 - 8.0_f64 / 27.0_f64 * t11593 * t17208 - t13040 - t13042 - t13049 + t13062 - t446 * t17357 / 3.0_f64 + t17360 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t17362 + 4.0_f64 / 27.0_f64 * t13075 + t13084 - 2.0_f64 / 9.0_f64 * t1901 * t17366 - 4.0_f64 / 9.0_f64 * t1901 * t17370;
    t17373
}
