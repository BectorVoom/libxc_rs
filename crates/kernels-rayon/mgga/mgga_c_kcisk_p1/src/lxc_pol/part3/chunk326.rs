//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 326/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk326(t1589: f64, t1591: f64, t1586: f64, t1568: f64, t1572: f64, t1578: f64, t1580: f64, t1583: f64, t535: f64, t541: f64, t544: f64, t1334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1592 = t1589 * t1591;
    let t1593 = t1586 * t1592;
    let t1596 = 0.2698618307426597582e-1_f64 * t1568 * t541 - 0.71963154864709268853e-1_f64 * t1572 * t541 + t1578 + 0.89953943580886586067e-2_f64 * t1580 * t1583 - 0.2698618307426597582e-1_f64 * t535 * t1593;
    let t1597 = 1.0_f64 / t544;
    let t1598 = t1596 * t1597;
    let t1601 = 0.11607361111111111111e-2_f64 * t1334;
    (t1592, t1593, t1596, t1597, t1598, t1601)
}
