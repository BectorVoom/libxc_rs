//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1249/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1249(t1092: f64, t13321: f64, t27764: f64, t283: f64, t27792: f64, t27836: f64, t100319: f64, t5302: f64, t93089: f64, t1800: f64, t27763: f64, t4772: f64) -> (f64, f64, f64, f64) {
    let t100383 = t1092 * t13321 * t283 * t27764;
    let t100386 = t1092 * t27836 * t27792;
    let t100389 = t5302 * t93089 * t100319;
    let t100398 = t1092 * t27763 * t1800 * t4772;
    (t100383, t100386, t100389, t100398)
}
