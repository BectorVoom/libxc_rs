//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 429/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk429(t1503: f64, t1556: f64, t1625: f64, t1629: f64, t1636: f64, t187: f64, t633: f64, t449: f64, t828: f64, t89: f64) -> (f64, f64, f64) {
    let t1640 = t1503 - t1556 + t187 * (t1625 * t633 - t1629 * t1636 - t1503 + t1556);
    let t1641 = t449 * t1640;
    let t1646 = -t89 - t828;
    (t1640, t1641, t1646)
}
