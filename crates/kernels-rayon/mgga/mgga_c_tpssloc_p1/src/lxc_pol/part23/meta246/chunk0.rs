//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 905/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk905(t18371: f64, t3577: f64, t248: f64, t3570: f64, t6219: f64, t1213: f64, t3521: f64, t5975: f64, t1227: f64, t1409: f64, t15701: f64, t3450: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18372 = t3577 * t18371;
    let t18375 = t248 * t3570 * t6219;
    let t18376 = t1213 * t18375;
    let t18392 = t248 * t3521 * t5975;
    let t18393 = t1227 * t18392;
    let t18395 = t15701 * t1409;
    let t18409 = t3450 * t5398;
    (t18372, t18375, t18376, t18392, t18393, t18395, t18409)
}
