//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 874/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk874(t13617: f64, t701: f64, t13593: f64, t13596: f64, t13601: f64, t13603: f64, t13607: f64, t13611: f64, t13614: f64, t9639: f64, t9642: f64, t9648: f64) -> (f64, f64) {
    let t13618 = t701 * t13617;
    let t13623 = 0.38306165027777777778e-1_f64 * t13593 - 0.85124811172839506173e-2_f64 * t13596 + t13601 + 0.85124811172839506173e-2_f64 * t13603 + 0.19862455940329218107e-1_f64 * t13607 - 0.3404992446913580247e-1_f64 * t13611 - 0.12768721675925925926e-1_f64 * t13614 + 0.51074886703703703704e-1_f64 * t13618 - 0.28374937057613168724e-2_f64 * t9639 + 0.21281202793209876543e-2_f64 * t9648 + 0.28374937057613168724e-2_f64 * t9642;
    (t13618, t13623)
}
