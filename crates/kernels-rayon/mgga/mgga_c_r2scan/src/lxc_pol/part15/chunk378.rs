//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 378/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk378(t424: f64, t457: f64, t41: f64, t410: f64, t425: f64, t1356: f64, t1378: f64, t1387: f64, t1389: f64, t1413: f64, t1418: f64, t1421: f64, t1424: f64, t1511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1512 = t424 * t457;
    let t1513 = t41 * t1512;
    let t1514 = 2.0_f64 * t1513;
    let t1515 = t410 * t425;
    let t1516 = 8.0_f64 * t1515;
    let t1517 = -t1387 - t1389 - t1413 + t1418 + t1421 - t1424 + t1511 + t1378 + t1514 - t1516 - t1356;
    (t1512, t1513, t1514, t1515, t1516, t1517)
}
