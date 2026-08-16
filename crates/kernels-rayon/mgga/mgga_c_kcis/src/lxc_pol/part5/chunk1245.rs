//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1245/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1245(t169: f64, t174: f64, t171: f64, t18443: f64, t20828: f64, t20833: f64, t2633: f64, t4510: f64, t829: f64, t13014: f64, t6281: f64, t2641: f64, t6284: f64, t176: f64, t18431: f64, t4518: f64, t833: f64, zeta_threshold: f64) -> (f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t20839 = piecewise3(t170, 0.0_f64, -8.0_f64 / 27.0_f64 * t20828 * t829 + 16.0_f64 / 9.0_f64 * t4510 * t2633 + 4.0_f64 / 9.0_f64 * t20833 * t829 + 4.0_f64 / 3.0_f64 * t171 * t18443);
    let t20840 = t13014 * t6281;
    let t20845 = t2641 * t6284;
    let t20851 = piecewise3(t175, 0.0_f64, -8.0_f64 / 27.0_f64 * t20840 * t833 - 16.0_f64 / 9.0_f64 * t4518 * t2633 + 4.0_f64 / 9.0_f64 * t20845 * t833 + 4.0_f64 / 3.0_f64 * t176 * t18431);
    (t20839, t20851)
}
