//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 479/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk479(t1515: f64, t468: f64, t963: f64, t2483: f64, t86: f64, t1521: f64, t1459: f64, t1463: f64, t1470: f64, t1480: f64, t1488: f64, t1513: f64, t1526: f64, t1529: f64, t1533: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2490 = 4.0_f64 * t1515;
    let t2491 = t963 * t468;
    let t2492 = 0.5848223622634646207e0_f64 * t2491;
    let t2493 = t2483 * t86;
    let t2494 = 0.19751673498613801407e-1_f64 * t2493;
    let t2495 = 0.18311447306006545054e-3_f64 * t1521;
    let t2496 = -t1459 + t1526 + t1513 - t2490 - t2492 - t1470 + t1480 + t1488 + t2494 + t1529 - t1463 - t2495 + t1533;
    (t2490, t2491, t2492, t2494, t2495, t2496)
}
