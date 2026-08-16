//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 346/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk346(t40: f64, t52: f64, t1409: f64, t185: f64, t707: f64, t73: f64, t76: f64, t145: f64, t157: f64, t182: f64, t767: f64, t771: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t1462 = t185 * t1409;
    let t1464 = 4.0_f64 * t707 * t1462;
    let t1467 = piecewise3(t146, 0.0_f64, 4.0_f64 / 3.0_f64 * t73 * t1409);
    let t1470 = piecewise3(t150, 0.0_f64, -4.0_f64 / 3.0_f64 * t76 * t1409);
    let t1471 = t1467 + t1470;
    let t1472 = t145 * t1471;
    let t1473 = t1472 * t185;
    let t1474 = t1471 * t157;
    let t1476 = 0.19751673498613801407e-1_f64 * t1474 * t182;
    let t1479 = piecewise3(t146, 0.0_f64, 2.0_f64 / 3.0_f64 * t767 * t1409);
    let t1482 = piecewise3(t150, 0.0_f64, -2.0_f64 / 3.0_f64 * t771 * t1409);
    let t1484 = t1479 / 2.0_f64 + t1482 / 2.0_f64;
    (t1462, t1464, t1471, t1472, t1473, t1474, t1476, t1484)
}
