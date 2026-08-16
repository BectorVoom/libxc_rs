//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 665/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk665(t701: f64, t9638: f64, t173: f64, t2442: f64, t2447: f64, t2451: f64, t191: f64, t2360: f64, t693: f64, t10: f64, t242: f64, t3050: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9639 = t701 * t9638;
    let t9641 = t173 * t2442;
    let t9642 = t701 * t9641;
    let t9644 = t173 * t2447;
    let t9645 = t701 * t9644;
    let t9647 = t173 * t2451;
    let t9648 = t701 * t9647;
    let t9651 = 1.0_f64 / t191 / t2360;
    let t9680 = t693 * t693;
    let t9681 = 1.0_f64 / t9680;
    let t9698 = t10 * t3050 * t242;
    (t9639, t9642, t9645, t9648, t9651, t9681, t9698)
}
