//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 244/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk244(t169: f64, t829: f64, t234: f64, t441: f64, t237: f64, t240: f64, t318: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t914 = piecewise3(t170, 0.0_f64, t829);
    let t915 = t234 * t914;
    let t916 = t915 * t441;
    let t920 = t237 * t318 * t240;
    (t915, t916, t920)
}
