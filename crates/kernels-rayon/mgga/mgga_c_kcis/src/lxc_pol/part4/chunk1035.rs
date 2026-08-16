//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1035/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1035(t169: f64, t4535: f64, t911: f64, t1300: f64, t6260: f64, t446: f64, t13003: f64, t1646: f64, t167: f64, t2629: f64, t160: f64, t171: f64, t2630: f64, t2635: f64, t4510: f64, t4513: f64, t740: f64, t829: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t13057 = t911 * t4535;
    let t13059 = t1300 * t6260;
    let t13060 = t446 * t13059;
    let t13062 = t13003 * t1646;
    let t13065 = t2629 * t167;
    let t13076 = piecewise3(t170, 0.0_f64, -8.0_f64 / 27.0_f64 * t13062 * t2630 + 16.0_f64 / 9.0_f64 * t13065 * t740 * t829 + 4.0_f64 / 9.0_f64 * t4510 * t2635 + 8.0_f64 / 3.0_f64 * t171 * t740 - 8.0_f64 * t4513 * t160);
    (t13057, t13060, t13076)
}
