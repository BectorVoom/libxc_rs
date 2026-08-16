//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 736/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk736(t117: f64, t1540: f64, t1614: f64, t570: f64, t1756: f64, t333: f64, t1587: f64, t1652: f64, t558: f64, t551: f64, t321: f64, t1763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30221 = t1540 * t117;
    let t30283 = t1614 * t570;
    let t30311 = t1756 * t333;
    let t30344 = t1587 * t570;
    let t30360 = t558 * t1652;
    let t30400 = t551 * t1652;
    let t30453 = t1756 * t321;
    let t30490 = t1763 * t333;
    (t30221, t30283, t30311, t30344, t30360, t30400, t30453, t30490)
}
