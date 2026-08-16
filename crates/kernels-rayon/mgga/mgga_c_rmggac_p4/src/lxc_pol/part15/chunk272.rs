//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 272/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk272(t1502: f64, t221: f64, t476: f64, t589: f64, t209: f64, t1228: f64, t612: f64, t1231: f64, t219: f64, t6: f64, t446: f64, t1392: f64, t489: f64, t490: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1503 = t221 * t1502;
    let t1508 = t589 * t476;
    let t1509 = t1508 * t209;
    let t1510 = t221 * t1509;
    let t1513 = t1228 * t612;
    let t1515 = t1231 * t219;
    let t1516 = t6 * t589;
    let t1518 = t1515 * t1516 * t446;
    let t1522 = t489 * t490 * t1392;
    (t1503, t1508, t1510, t1513, t1515, t1516, t1518, t1522)
}
