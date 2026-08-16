//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 863/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk863(t13421: f64, t677: f64, t25: f64, t3817: f64, t3762: f64, t1113: f64, t122: f64, t1095: f64, t2380: f64, t200: f64, t807: f64, t2427: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13422 = t677 * t13421;
    let t13425 = t3817 * t25;
    let t13426 = t13425 * t3762;
    let t13429 = t1113 * t122;
    let t13433 = t1095 * t2380;
    let t13434 = t13433 * t200;
    let t13435 = t807 * t13434;
    let t13442 = t2427 * t6;
    (t13422, t13426, t13429, t13433, t13434, t13435, t13442)
}
