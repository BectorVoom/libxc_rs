//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 855/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk855(t37355: f64, t82: f64, t13: f64, t7741: f64, t18: f64, t375: f64, t7760: f64, t1556: f64, t1569: f64) -> (f64, f64, f64, f64) {
    let t37356 = t82 * t37355;
    let t37387 = t7741 * t13;
    let t37388 = 1.0_f64 / t37387;
    let t37389 = t18 * t37388;
    let t37401 = t375 * t7760;
    let t37406 = 1.0_f64 / t1556 / t1569;
    (t37356, t37389, t37401, t37406)
}
