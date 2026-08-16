//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 431/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk431(t169: f64, t174: f64, t1650: f64, t176: f64, t1649: f64, t44: f64, t1646: f64, t234: f64, t441: f64, t330: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t1653 = piecewise3(t175, 0.0_f64, 4.0_f64 / 3.0_f64 * t176 * t1650);
    let t1655 = (t1649 + t1653) * t44;
    let t1657 = piecewise3(t170, 0.0_f64, t1646);
    let t1658 = t234 * t1657;
    let t1659 = t1658 * t441;
    let t1662 = t330 * t1646;
    (t1655, t1658, t1659, t1662)
}
