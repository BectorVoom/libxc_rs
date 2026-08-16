//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 375/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk375(t1399: f64, t1402: f64, t1404: f64, t1407: f64, t1393: f64, t1396: f64, t401: f64, t384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1473 = 0.39862222222222222223e0_f64 * t1399;
    let t1474 = 0.68258333333333333333e-1_f64 * t1402;
    let t1475 = 0.13651666666666666667e0_f64 * t1404;
    let t1476 = 0.13692777777777777778e0_f64 * t1407;
    let t1477 = -0.42198333333333333333e0_f64 * t1393 + 0.84396666666666666666e0_f64 * t1396 + t1473 + t1474 + t1475 + t1476;
    let t1478 = t1477 * t401;
    let t1479 = t384 * t1478;
    let t1480 = 1.0_f64 * t1479;
    (t1473, t1474, t1475, t1476, t1477, t1478, t1480)
}
