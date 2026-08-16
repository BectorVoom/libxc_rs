//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 845/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk845(t1952: f64, t3413: f64, t12633: f64, t12637: f64, t12665: f64, t13030: f64, t13136: f64, t13180: f64, t13228: f64, t13230: f64, t13234: f64, t149: f64, t165: f64, t3313: f64, t614: f64) -> f64 {
    let t13239 = t1952 * t3413;
    let t13245 = -t13228 * t149 - t13234 * t165 - 2.0_f64 * t13239 * t165 - 2.0_f64 * t3313 * t614 - 4.0_f64 * t12633 - 2.0_f64 * t12637 + 4.0_f64 * t12665 - 2.0_f64 * t13030 - 2.0_f64 * t13136 + 8.0_f64 * t13180 + 2.0_f64 * t13230;
    t13245
}
