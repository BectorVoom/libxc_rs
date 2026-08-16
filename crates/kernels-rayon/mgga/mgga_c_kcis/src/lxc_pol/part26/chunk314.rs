//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 314/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk314(t169: f64, t1503: f64, t1556: f64, t1625: f64, t1629: f64, t1636: f64, t187: f64, t633: f64, t828: f64, t89: f64, t171: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t1640 = t1503 - t1556 + t187 * (t1625 * t633 - t1629 * t1636 - t1503 + t1556);
    let t1646 = -t89 - t828;
    let t1649 = piecewise3(t170, 0.0_f64, 4.0_f64 / 3.0_f64 * t171 * t1646);
    let t1650 = -t1646;
    (t1640, t1646, t1649, t1650)
}
