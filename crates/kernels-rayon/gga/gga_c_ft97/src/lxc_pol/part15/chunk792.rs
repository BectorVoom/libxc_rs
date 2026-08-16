//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 792/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk792(t21196: f64, t3910: f64, t21404: f64, t9896: f64, t21408: f64, t2493: f64, t13335: f64, t13680: f64, t18283: f64, t18286: f64, t18303: f64, t18305: f64, t18314: f64, t18316: f64, t462: f64, t9935: f64) -> (f64, f64, f64, f64) {
    let t21607 = t3910 * t21196;
    let t21610 = t9896 * t21404;
    let t21613 = t2493 * t21408;
    let t21623 = -2.0_f64 / 3.0_f64 * t462 * t21607 - 2.0_f64 * t462 * t21610 - 2.0_f64 * t462 * t21613 + t18286 / 3.0_f64 + t18314 - t9935 - 4.0_f64 / 9.0_f64 * t13335 + 2.0_f64 / 9.0_f64 * t18303 - 2.0_f64 / 3.0_f64 * t18305 - 4.0_f64 / 3.0_f64 * t13680 - 2.0_f64 * t18316 - 2.0_f64 / 3.0_f64 * t18283;
    (t21607, t21610, t21613, t21623)
}
