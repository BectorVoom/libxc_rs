//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 989/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk989(t169: f64, t13003: f64, t6272: f64, t2629: f64, t6276: f64, t171: f64, t18443: f64, t2633: f64, t4510: f64, t829: f64, t13014: f64, t6281: f64, t2641: f64, t6284: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t20828 = t13003 * t6272;
    let t20833 = t2629 * t6276;
    let t20839 = piecewise3(t170, 0.0_f64, -8.0_f64 / 27.0_f64 * t20828 * t829 + 16.0_f64 / 9.0_f64 * t4510 * t2633 + 4.0_f64 / 9.0_f64 * t20833 * t829 + 4.0_f64 / 3.0_f64 * t171 * t18443);
    let t20840 = t13014 * t6281;
    let t20845 = t2641 * t6284;
    (t20839, t20840, t20845)
}
