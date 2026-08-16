//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 832/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk832(t12001: f64, t3471: f64, t1060: f64, t1647: f64, t569: f64, t13040: f64, t13042: f64, t13045: f64, t13049: f64, t13051: f64, t13055: f64, t13058: f64, t13062: f64, t13067: f64, t13072: f64, t1901: f64, t446: f64, t9321: f64, t9340: f64, t9342: f64) -> f64 {
    let t13075 = t12001 * t3471;
    let t13078 = t569 * t1060 * t1647;
    let t13081 = 8.0_f64 / 27.0_f64 * t9321 - t13040 - t13042 - 2.0_f64 / 9.0_f64 * t1901 * t13045 - t13049 - 2.0_f64 / 9.0_f64 * t1901 * t13051 - 2.0_f64 / 3.0_f64 * t1901 * t13055 + 4.0_f64 / 3.0_f64 * t446 * t13058 + t13062 + 2.0_f64 / 9.0_f64 * t9340 + 2.0_f64 / 9.0_f64 * t9342 - 2.0_f64 / 3.0_f64 * t446 * t13067 - 2.0_f64 * t446 * t13072 + 22.0_f64 / 27.0_f64 * t13075 + 2.0_f64 / 9.0_f64 * t446 * t13078;
    t13081
}
