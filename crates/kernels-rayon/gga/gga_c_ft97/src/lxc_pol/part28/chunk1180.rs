//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1180/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1180(t1359: f64, t6723: f64, t1349: f64, t138420: f64, t1389: f64, t139159: f64, t139171: f64, t139179: f64, t147856: f64, t148120: f64, t148205: f64, t149191: f64, t1969: f64, t26546: f64, t28: f64, t32686: f64, t32876: f64, t3408: f64, t379: f64, t5772: f64, t5778: f64, t6580: f64, t6589: f64, t7309: f64, t9073: f64, t925: f64) -> f64 {
    let t149419 = t1359 * t6723;
    let t149432 = -12.0_f64 * t147856 - t32686 * t6589 / 3.0_f64 + t139159 / 9.0_f64 + t5772 * t9073 * t138420 * t925 / 9.0_f64 - t7309 * t26546 / 3.0_f64 + t6580 * t32876 / 6.0_f64 - t139171 / 9.0_f64 - t139179 / 9.0_f64 - t5772 * t1969 * t149419 * t379 / 9.0_f64 + 8.0_f64 * t148205 + 8.0_f64 * t149191 + 8.0_f64 * t148120 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t5778 * t1389 * t3408;
    t149432
}
