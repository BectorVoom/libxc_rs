//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 771/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk771(t1551: f64, t1632: f64, t551: f64, t574: f64, t1541: f64, t545: f64, t548: f64, t2080: f64, t780: f64, t1234: f64, t566: f64, t110: f64, t6189: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6445 = t551 * t1632 * t1551;
    let t6446 = t574 * t6445;
    let t6448 = t545 * t1541;
    let t6449 = t6448 * t548;
    let t6455 = t2080 * t780;
    let t6457 = t1632 * t1234;
    let t6458 = t551 * t6457;
    let t6459 = t566 * t6458;
    let t6461 = t6189 * t110;
    (t6446, t6448, t6449, t6455, t6457, t6459, t6461)
}
