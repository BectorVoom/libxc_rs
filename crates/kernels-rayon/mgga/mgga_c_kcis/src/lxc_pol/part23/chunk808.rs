//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 808/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk808(t169: f64, t2628: f64, t174: f64, t2640: f64, t1646: f64, t167: f64, t2629: f64, t160: f64, t171: f64, t2630: f64, t2635: f64, t4510: f64, t4513: f64, t740: f64, t829: f64, zeta_threshold: f64) -> (f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t13003 = 1.0_f64 / t2628 / t169;
    let t13014 = 1.0_f64 / t2640 / t174;
    let t13062 = t13003 * t1646;
    let t13065 = t2629 * t167;
    let t13076 = piecewise3(t170, 0.0_f64, -8.0_f64 / 27.0_f64 * t13062 * t2630 + 16.0_f64 / 9.0_f64 * t13065 * t740 * t829 + 4.0_f64 / 9.0_f64 * t4510 * t2635 + 8.0_f64 / 3.0_f64 * t171 * t740 - 8.0_f64 * t4513 * t160);
    (t13014, t13076)
}
