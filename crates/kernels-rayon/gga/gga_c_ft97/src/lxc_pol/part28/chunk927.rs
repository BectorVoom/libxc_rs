//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 927/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk927(t582: f64, t5935: f64, t6685: f64, t157: f64, t40266: f64, t27015: f64, t50249: f64, t604: f64, t6615: f64, t1391: f64, t9114: f64, t1378: f64, t526: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106300 = t582 * t5935;
    let t106551 = t582 * t6685;
    let t106555 = t40266 * t157;
    let t106565 = t50249 * t27015;
    let t106573 = t604 * t6615;
    let t106619 = t9114 * t1391;
    let t106623 = t526 * t1378;
    (t106300, t106551, t106555, t106565, t106573, t106619, t106623)
}
