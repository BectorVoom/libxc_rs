//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1077/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1077(t169: f64, t18443: f64, t234: f64, t441: f64, t233: f64, t1641: f64, t6888: f64, t6295: f64, t911: f64, t6883: f64, t915: f64, t1881: f64, t6261: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t18444 = piecewise3(t170, 0.0_f64, t18443);
    let t18445 = t234 * t18444;
    let t18446 = t18445 * t441;
    let t18447 = t233 * t18446;
    let t18449 = t6888 * t1641;
    let t18451 = t911 * t6295;
    let t18453 = t915 * t6883;
    let t18454 = t233 * t18453;
    let t18456 = t1881 * t6261;
    (t18447, t18449, t18451, t18454, t18456)
}
